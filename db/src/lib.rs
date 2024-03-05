#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

pub mod error;
pub mod models;
#[cfg(test)] mod tests;
mod utils;

use anyhow::Context;
use error::DbError;
use models::{item::Item, user::User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type DbPool = sqlx::postgres::PgPool;
pub type DbResult<T> = Result<T, DbError>;

pub async fn migrate(pool: &DbPool) -> Result<(), DbError> {
  sqlx::migrate!("../db/migrations").run(pool).await?;
  Ok(())
}

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

pub async fn get_item_by_id(pool: &sqlx::Pool<sqlx::Postgres>, item_id: Uuid) -> DbResult<Option<Item>> {
  let query = r#"
    SELECT
    id,
    by,
    title,
    item_type AS \"item_type: item_type\",
    url,
    domain,
    text,
    points,
    score,
    comment_count,
    item_category AS \"item_category: item_category\",
    created,
    dead
    FROM users WHERE id = {item_id}"#;

  sqlx::query_as(query)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}
