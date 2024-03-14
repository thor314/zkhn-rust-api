use uuid::Uuid;

use crate::{
  error::DbError,
  models::{comment::Comment, user::User},
  DbPool, DbResult,
};

pub async fn get_user_by_username(pool: &DbPool, username: &str) -> DbResult<Option<User>> {
  sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}
