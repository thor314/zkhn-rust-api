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

// pub async fn get_user_vote_by_content_id(
//   pool: &DbPool,
//   username: &str,
//   content_id: Uuid,
// ) -> DbResult<Option<UserVote>> {
//   sqlx::query_as!(
//     UserVote,
//     "SELECT * FROM user_votes WHERE content_id =  and username = ",
//     content_id,
//     username
//   )
//   .fetch_optional(pool)
//   .await
//   .map_err(DbError::from)
// }
