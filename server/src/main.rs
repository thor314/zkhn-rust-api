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

async fn index() -> &'static str { "Hello, world!" }

async fn error_handler() -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

#[shuttle_runtime::main]
async fn main(
  #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
  #[shuttle_shared_db::Postgres] pool: sqlx::PgPool
) -> shuttle_axum::ShuttleAxum {
  utils::setup(&secret_store).unwrap();

  // let shared_state = SharedState { pool };

  let router = Router::new();
  // .route("/", get(index))
  // .route("/comments", get(index)) // todo
  // .route("/-1/error", get(error_handler))
  // .route("/-1/health", get(|| async { StatusCode::OK }))
  // .with_state(shared_state);

  Ok(router.into())
}
