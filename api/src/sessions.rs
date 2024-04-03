//! auth layer depends on this, don't move to server
use axum_login::tower_sessions::{cookie::time::Duration, Expiry, SessionManagerLayer};
use db::DbPool;
use tower_sessions::{cookie::Key, service::SignedCookie, ExpiredDeletion};
use tower_sessions_sqlx_store::PostgresStore;

pub(super) type MySessionManagerLayer = SessionManagerLayer<PostgresStore, SignedCookie>;

/// create the session store and migrate the session table in the database.
///
/// tower-sessions docs: https://docs.rs/tower-sessions/latest/tower_sessions/service/struct.SessionManagerLayer.html
/// MDN cookie docs: https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies
pub(super) async fn create_migrate_session_layer(pool: DbPool, key: Key) -> MySessionManagerLayer {
  let session_store = tower_sessions_sqlx_store::PostgresStore::new(pool);
  session_store.migrate().await.unwrap();

  // create a deletion task to continuously delete expired sessions
  let _deletion_task = tokio::task::spawn(
    session_store.clone().continuously_delete_expired(tokio::time::Duration::from_secs(60)),
  );

  // create the session layer. SessionManagerLayer does the heavy lifting in tower-sessions
  SessionManagerLayer::new(session_store)
    .with_secure(false)
    .with_expiry(Expiry::OnInactivity(Duration::days(1)))
    .with_signed(key)
}
