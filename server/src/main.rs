#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod error;
#[cfg(test)] mod tests;
mod utils;

use axum::{
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use db::DbPool;
use error::MyError;
use shuttle_secrets::SecretStore;
use tracing::info;

#[shuttle_runtime::main]
async fn main(
  #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
  #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> shuttle_axum::ShuttleAxum {
  utils::setup(&secret_store).unwrap();

  let router = api::api_router(pool).await;

  Ok(router.into())
}
