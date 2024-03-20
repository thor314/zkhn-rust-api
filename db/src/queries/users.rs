use futures::TryFutureExt;
use sqlx::postgres::PgQueryResult;
use tracing::{info, instrument, warn};
use uuid::Uuid;

use crate::{
  error::DbError,
  models::{comment::Comment, item::Item, user::User},
  About, DbPool, DbResult, Email, Username,
};

pub async fn get_user(pool: &DbPool, username: &str) -> DbResult<Option<User>> {
  info!("get_user function called w username: {username}");
  sqlx::query_as!(
    User,
    "SELECT username, 
            password_hash, 
            auth_token, 
            auth_token_expiration, 
            reset_password_token, 
            reset_password_token_expiration, 
            email as \"email: Email\", 
            created, 
            karma, 
            about as \"about: About\", 
            show_dead, 
            is_moderator, 
            shadow_banned, 
            banned 
     FROM users WHERE username = $1",
    username
  )
  .fetch_optional(pool)
  .await
  .map_err(DbError::from)
}

pub async fn create_user(pool: &DbPool, new_user: &User) -> DbResult<()> {
  info!("create_user with: {new_user:?}");
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
    karma,
    about,
    show_dead,
    is_moderator,
    shadow_banned,
    banned
  ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
    username,
    password_hash,
    auth_token,
    auth_token_expiration,
    reset_password_token,
    reset_password_token_expiration,
    email.map(|s| s.0),
    created.0,
    karma,
    about.map(|s| s.0),
    show_dead,
    is_moderator,
    shadow_banned,
    banned,
  )
  .execute(&mut *tx)
  .await;

  if let Err(e) = &result {
    // unwrap is safe; error is always db error kinded
    if e.as_database_error().expect("expected db error").is_unique_violation() {
      tx.rollback().await?;
      warn!("user already exists");
      return Ok(());
    } else {
      tracing::error!("error creating user: {e}");
    }
  }
  let _ = result?;

  tx.commit().await?;
  // todo
  Ok(())
}

pub async fn delete_user(pool: &DbPool, username: &str) -> DbResult<()> {
  let result =
    sqlx::query!("DELETE FROM users WHERE username = $1", username).execute(pool).await?;

  if result.rows_affected() == 0 {
    warn!("user {username} does not exist");
  } else {
    info!("user {username} deleted");
  }

  Ok(())
}

pub async fn get_user_comments(pool: &DbPool, username: &str) -> DbResult<Vec<Comment>> {
  sqlx::query_as!(Comment, "SELECT * FROM comments WHERE username = $1", username)
    .fetch_all(pool)
    .await
    .map_err(DbError::from)
}

pub async fn get_user_items(pool: &DbPool, username: &str) -> DbResult<Vec<Item>> {
  sqlx::query_as!(Item, "SELECT * FROM items WHERE username = $1", username)
    .fetch_all(pool)
    .await
    .map_err(DbError::from)
}

// todo: make generic to update other user fields
pub async fn update_user_about(
  pool: &DbPool,
  username: &Username,
  about: &About,
) -> DbResult<PgQueryResult> {
  sqlx::query!("UPDATE users SET about = $1 WHERE username = $2", about.0, username.0)
    .execute(pool)
    .await
    .map_err(DbError::from)
}
