mod cors;
mod error;
mod utils;

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
  db::migrate(&pool).await;

  debug!("Initializing router...");
  // let analytics_key = secret_store.get("ANALYTICS_API_KEY");
  let session_key =
    secret_store.get("SESSION_KEY").map(|s| Key::from(s.as_bytes())).unwrap_or_else(|| {
      warn!("using insecure key generation");
      Key::generate()
    });

  let app = api::app(pool, session_key).await.expect("failed to build app")
    .layer(cors::cors_layer())
    // todo(prod)
    // .layer(Analytics::new(analytics_key.unwrap_or("".to_string()))) // must precede auth
    ;

  info!("🚀🚀🚀 see http://localhost:8000/docs/rapidoc for api docs 🚀🚀🚀");
  Ok(app.into())
}
