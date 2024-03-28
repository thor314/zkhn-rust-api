use futures::TryFutureExt;
use sqlx::postgres::PgQueryResult;
use tracing::{debug, info, instrument, warn};
use uuid::Uuid;

use crate::{
  error::DbError,
  models::{comment::Comment, item::Item, user::User},
  About, AuthToken, CommentText, DbPool, DbResult, Email, Password, PasswordHash,
  ResetPasswordToken, Timestamp, Title, Username,
};

pub async fn get_user(pool: &DbPool, username: &Username) -> DbResult<Option<User>> {
  debug!("get_user function called w username: {username}");
  sqlx::query_as!(
    User,
    "SELECT username as \"username: Username\", 
            password_hash as \"password_hash: PasswordHash\", 
            auth_token as \"auth_token: AuthToken\", 
            auth_token_expiration as \"auth_token_expiration: Timestamp\", 
            reset_password_token as \"reset_password_token: ResetPasswordToken\", 
            reset_password_token_expiration as \"reset_password_token_expiration: Timestamp\",  
            email as \"email: Email\", 
            created, 
            karma, 
            about as \"about: About\", 
            show_dead, 
            is_moderator, 
            shadow_banned, 
            banned 
     FROM users WHERE username = $1",
    username.0
  )
  .fetch_optional(pool)
  .await
  .map_err(DbError::from)
}

/// Create a new user in the database.
/// If the username already exists, return Recoverable::DbEntryAlreadyExists.
pub async fn create_user(pool: &DbPool, new_user: &User) -> DbResult<()> {
  debug!("create_user with: {new_user:?}");
  let mut tx = pool.begin().await?;

  let User {
    username,
    password_hash,
    auth_token,
    auth_token_expiration,
    reset_password_token,
    reset_password_token_expiration,
    email,
    created,
    karma,
    about,
    show_dead,
    is_moderator,
    shadow_banned,
    banned,
  } = new_user.clone();

  let result = sqlx::query!(
    "INSERT INTO users
    ( username,
    password_hash,
    auth_token,
    auth_token_expiration,
    reset_password_token,
    reset_password_token_expiration,
    email,
    created,
    about
  ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    username.0,
    password_hash.0,
    auth_token.map(|s| s.0),
    auth_token_expiration.map(|t| t.0),
    reset_password_token.map(|s| s.0),
    reset_password_token_expiration.map(|t| t.0),
    email.map(|s| s.0),
    created.0,
    about.map(|s| s.0),
  )
  .execute(&mut *tx)
  .await;

  if let Err(e) = &result {
    // unwrap is safe; error is always db error kinded
    if e.as_database_error().expect("expected db error").is_unique_violation() {
      tx.rollback().await?;
      warn!("user already exists");
      return Err(DbError::Conflict);
    } else {
      tracing::error!("error creating user: {e}");
    }
  }
  let _ = result?;

  tx.commit().await?;
  // todo
  Ok(())
}

pub async fn delete_user(pool: &DbPool, username: &Username) -> DbResult<()> {
  debug!("delete_user with: {username}");
  let result =
    sqlx::query!("DELETE FROM users WHERE username = $1", username.0).execute(pool).await?;

  if result.rows_affected() == 0 {
    warn!("user {username} does not exist");
    Err(DbError::NotFound)
  } else {
    info!("user {username} deleted");
    Ok(())
  }
}

// pub async fn get_user_comments(pool: &DbPool, username: &Username) -> DbResult<Vec<Comment>> {
//   debug!("get_user_comments with: {username}");
//   sqlx::query_as!(
//     Comment,
//     "SELECT 
//     id,
//     username as \"username: Username\",
//     parent_item_id,
//     parent_item_title as \"parent_item_title: Title\",
//     comment_text as \"comment_text: CommentText\",
//     is_parent,
//     root_comment_id,
//     parent_comment_id,
//     children_count,
//     points,
//     created,
//     dead
//    FROM comments WHERE username = $1",
//     username.0
//   )
//   .fetch_all(pool)
//   .await
//   .map_err(DbError::from)
// }

// pub async fn get_user_items(pool: &DbPool, username: &Username) -> DbResult<Vec<Item>> {
//   debug!("get_user_items with: {username}");
//   sqlx::query_as!(
//     Item,
//     "SELECT 
//       id,
//       username as \"username: Username\",    
//       title as \"title: Title\",  
//       item_type,   
//       url,         
//       domain,      
//       text,        
//       points,      
//       score,       
//       comment_count,
//       item_category,
//       created,     
//       dead
//     FROM items WHERE username = $1",
//     username.0
//   )
//   .fetch_all(pool)
//   .await
//   .map_err(DbError::from)
// }

pub async fn update_user_about(
  pool: &DbPool,
  username: &Username,
  about: &About,
) -> DbResult<PgQueryResult> {
  debug!("update_user_about with: {username}");
  sqlx::query!("UPDATE users SET about = $1 WHERE username = $2", about.0, username.0)
    .execute(pool)
    .await
    .map_err(DbError::from)
}

pub async fn update_user_email(
  pool: &DbPool,
  username: &Username,
  email: &Email,
) -> DbResult<PgQueryResult> {
  debug!("update_user_email with: {username}");
  sqlx::query!("UPDATE users SET email = $1 WHERE username = $2", email.0, username.0)
    .execute(pool)
    .await
    .map_err(DbError::from)
}

/// Set the user's auth token and expiration in the database to `None`.
pub async fn logout_user(pool: &DbPool, username: &Username) -> DbResult<PgQueryResult> {
  debug!("logout_user with: {username}");
  sqlx::query!(
    "UPDATE users SET auth_token = NULL, auth_token_expiration = NULL WHERE username = $1",
    username.0
  )
  .execute(pool)
  .await
  .map_err(DbError::from)
}

pub async fn update_user_auth_token(
  pool: &DbPool,
  username: &Username,
  auth_token: &AuthToken,
  auth_token_expiration: &Timestamp,
) -> DbResult<()> {
  debug!("update_user_auth_token with: {username}");
  sqlx::query!(
    "UPDATE users SET auth_token = $1, auth_token_expiration = $2 WHERE username =
  $3",
    auth_token.0,
    auth_token_expiration.0,
    username.0
  )
  .execute(pool)
  .await
  .map_err(DbError::from)?;

  Ok(())
}

pub async fn update_user_password_token(
  pool: &DbPool,
  username: &Username,
  reset_password_token: &AuthToken,
  reset_password_token_expiration: &Timestamp,
) -> DbResult<()> {
  debug!("update_user_password_token with: {username}");
  sqlx::query!(
    "UPDATE users SET reset_password_token = $1, reset_password_token_expiration = $2 WHERE \
     username =
  $3",
    reset_password_token.0,
    reset_password_token_expiration.0,
    username.0
  )
  .execute(pool)
  .await
  .map_err(DbError::from)?;

  Ok(())
}

pub async fn update_user_password(
  pool: &sqlx::Pool<sqlx::Postgres>,
  username: &Username,
  new_password_hash: &PasswordHash,
) -> DbResult<()> {
  debug!("update_user_password with: {username}");
  sqlx::query!(
    "UPDATE users SET password_hash = $1, auth_token = NULL, auth_token_expiration = NULL WHERE \
     username =
  $2",
    new_password_hash.0,
    username.0,
  )
  .execute(pool)
  .await
  .map_err(DbError::from)?;

  Ok(())
}
