#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod error;
mod models;
mod schema;
#[cfg(test)] mod tests;
mod utils;

use axum::{
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use diesel_async::{
  pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
  AsyncConnection, AsyncPgConnection,
};
use error::MyError;
use shuttle_secrets::SecretStore;
use tracing::info;

async fn index() -> &'static str { "Hello, world!" }

async fn error_handler() -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

#[derive(Clone)]
pub struct SharedState {
  pub pool: Pool<AsyncPgConnection>,
}

#[shuttle_runtime::main]
async fn main(
  #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
  // shuttle will provision the database connection
  #[shuttle_shared_db::Postgres(
    local_uri = "postgres://postgres:{secrets.DB_PASSWORD}@localhost:{secrets.DB_PORT}/postgres"
  )]
  conn_str: String,
) -> shuttle_axum::ShuttleAxum {
  utils::setup(&secret_store).unwrap();

  let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(conn_str);
  let pool = Pool::builder(config).build().unwrap();
  let shared_state = SharedState { pool };

  let router = Router::new()
    .route("/", get(index))
    .route("/comments", get(index)) // todo
    .route("/-1/error", get(error_handler))
    .route("/-1/health", get(|| async { StatusCode::OK }))
    .with_state(shared_state);

  Ok(router.into())
}

pub async fn establish_connection(secrets: &SecretStore) -> AsyncPgConnection {
  let db_url = secrets.get("DATABASE_URL").expect("DATABASE_URL must be set");

  AsyncPgConnection::establish(&db_url)
    .await
    .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
