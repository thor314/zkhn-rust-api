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

pub async fn delete_user(pool: &DbPool, username: &str) -> DbResult<()> {
  sqlx::query!("DELETE FROM users WHERE username = $1", username)
    .execute(pool)
    .await
    .map_err(DbError::from)
    .map(|_| ())
}

pub async fn get_user_comments(pool: &DbPool, username: String) -> DbResult<Vec<Comment>> {
  sqlx::query_as!(Comment, "SELECT * FROM comments WHERE by = $1", username)
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
