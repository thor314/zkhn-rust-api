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
use tower_sessions::cookie::Key;
use tracing::{debug, info, warn};

pub type ServerResult<T> = Result<T, ServerError>;

#[shuttle_runtime::main]
async fn main(
  #[shuttle_runtime::Secrets] secret_store: shuttle_runtime::SecretStore,
  #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
  debug!("pool info: {:?}", pool);
  utils::setup(&secret_store).unwrap();
  db::migrate(&pool).await.unwrap();

  debug!("Initializing router...");
  // let analytics_key = secret_store.get("ANALYTICS_API_KEY");
  let session_key =
    secret_store.get("SESSION_KEY").map(|s| Key::from(s.as_bytes())).unwrap_or_else(|| {
      warn!("using insecure key generation");
      Key::generate()
    });

  let app = api::app(pool, session_key).await.expect("failed to build app");

  info!("🚀🚀🚀 see http://localhost:8000/docs/rapidoc for api docs 🚀🚀🚀");
  Ok(app.into())
}
