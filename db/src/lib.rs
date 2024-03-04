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
use models::user::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type DbPool = sqlx::postgres::PgPool;
pub type DbResult<T> = Result<T, DbError>;

pub async fn migrate(pool: &DbPool) -> Result<(), DbError> {
  sqlx::migrate!("../db/migrations").run(pool).await?;
  Ok(())
}

// // todo: move to user-queries
// pub async fn get_user_by_id(db_pool: &DbPool, id: Uuid) -> DbResult<Option<User>> {
//   sqlx::query_as!(User, "select * from users where id = id").fetch_optional(db_pool).await.map_err(DbError::from)
//   // sqlx::query_as!(User, "select user from users where id = ?", id).fetch_optional(db_pool).await.map_err(DbError::from)
//   // sqlx::query_as!(Option<User>, "select user from users where id = ?", id).fetch_optional(db_pool).await.map_err(DbError::from)
//   // sqlx::query_as(r"select user from users where id = ?").bind(id).fetch_optional(db_pool).await.map_err(DbError::from)
// }

// pub async fn get_user_by_username(db_pool: &DbPool, username: &str) -> DbResult<Option<User>> {
//   sqlx::query_as("select * from users where username = ? ")
//     .bind(username)
//     .fetch_optional(db_pool)
//     .await
//     .map_err(DbError::from)
//   // .expect("db failure")
// }
