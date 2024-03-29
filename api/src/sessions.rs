use axum_login::tower_sessions::{
  cookie::time::Duration, session_store, Expiry, SessionManagerLayer,
};
use db::DbPool;
use tower_sessions::{cookie::Key, service::SignedCookie, ExpiredDeletion};
use tower_sessions_sqlx_store::PostgresStore;
use tracing::info;

use crate::{
  auth::{get_auth_layer, AuthBackend},
  ApiResult,
};

pub(super) type MySessionManagerLayer = SessionManagerLayer<PostgresStore, SignedCookie>;

pub(super) async fn create_migrate_session_layer(pool: DbPool) -> ApiResult<MySessionManagerLayer> {
  // create the session store and migrate the database
  let session_store = tower_sessions_sqlx_store::PostgresStore::new(pool);
  session_store.migrate().await?;

  // create a deletion task to continuously delete expired sessions
  let deletion_task = tokio::task::spawn(
    session_store.clone().continuously_delete_expired(tokio::time::Duration::from_secs(60)),
  );

  // create the session layer
  let session_layer = SessionManagerLayer::new(session_store)
    .with_secure(false)
    .with_expiry(Expiry::OnInactivity(Duration::days(1)))
    // todo(prod): insecure to generate cryptographic key this way
    .with_signed(Key::generate());

  Ok(session_layer)
}
