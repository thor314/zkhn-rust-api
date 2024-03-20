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

use std::fmt;

use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, Decode, Encode};
use uuid::Uuid;

pub use crate::error::DbError;

pub type DbPool = sqlx::postgres::PgPool;
pub type DbResult<T> = Result<T, DbError>;

pub async fn migrate(pool: &DbPool) -> Result<(), DbError> {
  sqlx::migrate!("../db/migrations").run(pool).await?;
  Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Type)]
#[garde(transparent)]
#[repr(transparent)]
pub struct About(#[garde(ascii, length(min = 0, max = 400))] pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq, Type)]
#[garde(transparent)]
#[repr(transparent)]
pub struct Username(#[garde(ascii, length(min = 3, max = 25))] pub String);

impl fmt::Display for Username {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Type)]
#[garde(transparent)]
#[repr(transparent)]
pub struct Email(#[garde(email)] pub String);

impl std::fmt::Display for Email {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}
