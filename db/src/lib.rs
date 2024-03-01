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

use models::user::User;
use uuid::Uuid as Uid;

pub type DbPool = sqlx::postgres::PgPool;

pub async fn get_user_from_id(db_pool: &DbPool, id: Uid) -> Option<User> { todo!() }
