#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod error;
mod utils;

use anyhow::Context;
use error::ServerError;
use sqlx::PgPool;
use tracing::info;

pub type ServerResult<T> = Result<T, ServerError>;

#[shuttle_runtime::main]
async fn main(
  #[shuttle_runtime::Secrets] secret_store: shuttle_runtime::SecretStore,
  #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
  utils::setup(&secret_store).unwrap();
  info!("Migrating db...");
  info!("pool info: {:?}", pool);
  db::migrate(&pool).await.unwrap();
  info!("Initializing router...");

  info!("Building middleware layers...");
  let analytics_key = secret_store.get("ANALYTICS_API_KEY");
  let router = api::router(&pool, analytics_key).await.context("failed to build router").unwrap();

  info!("🚀🚀🚀 see http://localhost:8000/docs/rapidoc for api docs 🚀🚀🚀");
  Ok(router.into())
}
