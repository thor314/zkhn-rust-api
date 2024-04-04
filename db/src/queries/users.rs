use futures::TryFutureExt;
use sqlx::postgres::PgQueryResult;
use tracing::{debug, error, info, instrument, trace, warn};
use uuid::Uuid;

use crate::{
  error::DbError,
  models::{comment::Comment, item::Item, user::User},
  About, AuthToken, CommentText, DbPool, DbResult, Email, Password, PasswordHash,
  ResetPasswordToken, Timestamp, Title, Username,
};

pub async fn get_user(pool: &DbPool, username: &Username) -> DbResult<User> {
  trace!("get_user function called w username: {username}");
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
  .map(|r| r.ok_or(DbError::NotFound("user".into())))?
  .map_err(DbError::from)
}

/// Create a new user in the database.
pub async fn create_user(pool: &DbPool, new_user: &User) -> DbResult<()> {
  trace!("create_user with: {new_user:?}");
  let mut tx = pool.begin().await?;

  let User {
    username,
    password_hash,
    auth_token,
    auth_token_expiration,
    reset_password_token,
    reset_password_token_expiration,
    email,
    karma,
    ..
  } = new_user.clone();

  sqlx::query!(
    "INSERT INTO users
    ( username,
    password_hash,
    auth_token,
    auth_token_expiration,
    reset_password_token,
    reset_password_token_expiration,
    email
  ) VALUES ($1, $2, $3, $4, $5, $6, $7)",
    username.0,
    password_hash.0,
    auth_token.map(|s| s.0),
    auth_token_expiration.map(|t| t.0),
    reset_password_token.map(|s| s.0),
    reset_password_token_expiration.map(|t| t.0),
    email.map(|s| s.0),
  )
  .execute(&mut *tx)
  .await
  .map_err(DbError::from)?;

  tx.commit().await?;
  Ok(())
}

pub async fn update_user(
  pool: &DbPool,
  username: &Username,
  about: &Option<About>,
  email: &Option<Email>,
) -> DbResult<()> {
  let mut tx = pool.begin().await?;
  if let Some(about) = about {
    sqlx::query!("UPDATE users SET about = $1 WHERE username = $2", about.0, username.0)
      .execute(&mut *tx)
      .await
      .map_err(DbError::from)?;
  }
  if let Some(email) = email {
    sqlx::query!("UPDATE users SET email = $1 WHERE username = $2", email.0, username.0)
      .execute(&mut *tx)
      .await
      .map_err(DbError::from)?;
  }

  trace!("update_user with: {username}");
  Ok(tx.commit().await?)
}

pub async fn update_user_password_token(
  pool: &DbPool,
  username: &Username,
  reset_password_token: &AuthToken,
  reset_password_token_expiration: &Timestamp,
) -> DbResult<()> {
  trace!("update_user_password_token with: {username}");
  sqlx::query!(
    "UPDATE users SET 
    reset_password_token = $1, 
    reset_password_token_expiration = $2 
    WHERE username = $3",
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
  pool: &DbPool,
  username: &Username,
  new_password_hash: &PasswordHash,
) -> DbResult<()> {
  trace!("update_user_password with: {username}");
  sqlx::query!(
    "UPDATE users SET 
    password_hash = $1, 
    auth_token = NULL, 
    auth_token_expiration = NULL 
    WHERE username = $2",
    new_password_hash.0,
    username.0,
  )
  .execute(pool)
  .await
  .map_err(DbError::from)?;

  Ok(())
}

// /// Set the user's auth token and expiration in the database to `None`.
// pub async fn logout_user(pool: &DbPool, username: &Username) -> DbResult<PgQueryResult> {
//   trace!("logout_user with: {username}");
//   sqlx::query!(
//     "UPDATE users SET auth_token = NULL, auth_token_expiration = NULL WHERE username = $1",
//     username.0
//   )
//   .execute(pool)
//   .await
//   .map_err(DbError::from)
// }

// pub async fn update_user_auth_token(
//   pool: &DbPool,
//   username: &Username,
//   auth_token: &AuthToken,
//   auth_token_expiration: &Timestamp,
// ) -> DbResult<()> {
//   trace!("update_user_auth_token with: {username}");
//   sqlx::query!(
//     "UPDATE users SET auth_token = $1, auth_token_expiration = $2 WHERE username =
//   $3",
//     auth_token.0,
//     auth_token_expiration.0,
//     username.0
//   )
//   .execute(pool)
//   .await
//   .map_err(DbError::from)?;

//   Ok(())
// }

// pub async fn get_user_comments(pool: &DbPool, username: &Username) -> DbResult<Vec<Comment>> {
// trace!("get_user_comments with: {username}");
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
//   trace!("get_user_items with: {username}");
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
