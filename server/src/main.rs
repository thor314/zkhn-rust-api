#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod error;
#[cfg(test)] mod tests;
mod utils;

use anyhow::Context;
use db::DbPool;
use error::ServerError;
use sqlx::PgPool;
use tower_sessions::{session_store::ExpiredDeletion, Expiry, SessionManagerLayer};
// use tokio::{signal, task::AbortHandle};
use tower_sessions_sqlx_store::PostgresStore;

pub type ServerResult<T> = Result<T, ServerError>;

#[shuttle_runtime::main]
async fn main(
  #[shuttle_runtime::Secrets] secret_store: shuttle_runtime::SecretStore,
  #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
  tracing::info!("Starting server...");
  utils::setup(&secret_store).unwrap();
  tracing::info!("Migrating db...");
  db::migrate(&pool).await.unwrap();
  tracing::info!("Initializing router...");

  tracing::info!("Building middleware layers...");
  let analytics_key = secret_store.get("ANALYTICS_API_KEY");
  let router = api::router(&pool, analytics_key).await.context("failed to build router").unwrap();

  tracing::info!("🚀🚀🚀");
  Ok(router.into())
}
