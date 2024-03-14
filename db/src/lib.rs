#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod error;
pub mod models;
pub mod queries;
#[cfg(test)] mod tests;
mod utils;

use uuid::Uuid;

pub use crate::error::DbError;
use crate::{
  models::{
    comment::Comment, item::Item, user::User, user_favorite::UserFavorite, user_vote::UserVote,
  },
  utils::now,
};

pub type DbPool = sqlx::postgres::PgPool;
pub type DbResult<T> = Result<T, DbError>;

pub async fn migrate(pool: &DbPool) -> Result<(), DbError> {
  sqlx::migrate!("../db/migrations").run(pool).await?;
  Ok(())
}
