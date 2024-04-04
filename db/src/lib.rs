#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod error;
pub mod models;
pub mod queries;
mod types;
mod utils;

pub use crate::{error::*, types::*};

pub type DbPool = sqlx::postgres::PgPool;
pub type DbResult<T> = Result<T, DbError>;

/// the minimum points a comment can have
pub const MIN_COMMENT_POINTS: i32 = -4;

pub async fn migrate(pool: &DbPool) { sqlx::migrate!("../db/migrations").run(pool).await.unwrap(); }
