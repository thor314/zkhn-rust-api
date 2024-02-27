#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod api;
pub mod error;
#[cfg(test)] mod tests;
mod utils;

use axum::{
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use error::MyError;
use shuttle_secrets::SecretStore;
use tracing::info;

#[derive(Clone)]
pub struct SharedState {
  pub pool: Pool<AsyncPgConnection>,
}
