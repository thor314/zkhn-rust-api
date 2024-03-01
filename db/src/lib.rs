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

use error::DbError;
use models::user::User;
use uuid::Uuid;

pub type DbPool = sqlx::postgres::PgPool;

pub async fn migrate(pool: &DbPool) -> Result<(), DbError> {
  sqlx::migrate!("../db/migrations").run(pool).await?;
  Ok(())
}

pub async fn get_user_from_id(db_pool: &DbPool, id: Uuid) -> Option<User> { todo!() }
