use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

use crate::{
  error::DbError,
  models::{comment::Comment, user::User},
  DbPool, DbResult,
};

pub async fn get_user(pool: &DbPool, username: &str) -> DbResult<Option<User>> {
  sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn insert_user(pool: &DbPool, new_user: &User) -> DbResult<()> {
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

  sqlx::query!(
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
    auth_token.unwrap_or("".to_string()),
    auth_token_expiration.unwrap_or(0),
    reset_password_token.unwrap_or("".to_string()),
    reset_password_token_expiration.unwrap_or(0),
    email,
    created.0,
    karma,
    about.unwrap_or("".to_string()),
    show_dead,
    is_moderator,
    shadow_banned,
    banned,
  )
  .execute(&mut *tx)
  .await?;

  tx.commit().await?;
  // todo
  Ok(())
}

pub async fn delete_user(pool: &DbPool, username: &str) -> DbResult<()> {
  sqlx::query!("DELETE FROM users WHERE username = $1", username)
    .execute(pool)
    .await
    .map_err(DbError::from)
    .map(|_| ())
}

pub async fn get_user_comments(pool: &DbPool, username: &str) -> DbResult<Vec<Comment>> {
  sqlx::query_as!(Comment, "SELECT * FROM comments WHERE username = $1", username)
    .fetch_all(pool)
    .await
    .map_err(DbError::from)
}

pub async fn update_user_about(
  pool: &DbPool,
  username: &str,
  about: &str,
) -> DbResult<PgQueryResult> {
  sqlx::query!("UPDATE users SET about = $1 WHERE username = $2", about, username)
    .execute(pool)
    .await
    .map_err(DbError::from)
}
