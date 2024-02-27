#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod api;
mod auth;
pub mod error;
#[cfg(test)] mod tests;
mod utils;

use axum::{
  extract::Request,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use diesel_async::{
  pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
  AsyncPgConnection,
};
use error::MyError;
use tracing::info;

pub type DbPool = Pool<AsyncPgConnection>;

/// Access to the database
#[derive(Clone)]
pub struct SharedState {
  pub pool: DbPool,
}

// impl SharedState {
//   pub fn new() -> Self {
//     let conn_str = "todo";
//     let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(conn_str);
//     let pool = Pool::builder(config).build().unwrap();
//     Self { pool }
//   }
// }
