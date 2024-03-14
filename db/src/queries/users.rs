use uuid::Uuid;

use crate::{
  error::DbError,
  models::{comment::Comment, user::User},
  DbPool, DbResult,
};
// todo: move to user-queries
pub async fn get_user_by_id(pool: &DbPool, id: Uuid) -> DbResult<Option<User>> {
  sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id) // syntax error at end of input
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn get_user_by_username(pool: &DbPool, username: &str) -> DbResult<Option<User>> {
  sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}
