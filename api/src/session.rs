// https://docs.rs/tower-sessions/0.10.0/tower_sessions/index.html
// todo

use axum_login::tower_sessions::{
  cookie::time::Duration, ExpiredDeletion, Expiry, MemoryStore, SessionManagerLayer, SessionStore,
};
use tokio::task;
use tower_sessions_sqlx_store::PostgresStore;

use crate::DbPool;

// todo: standin mirror of
// https://github.com/maxcountryman/axum-login/blob/main/examples/sqlite/src/web/app.rs
// until I look into session management https://docs.rs/tower-sessions/0.10.0/tower_sessions/index.html
// pub async fn get_session_manager_layer(pool: &DbPool) -> SessionManagerLayer<impl SessionStore> {
//   // let session_store = SqliteStore::new(self.db.clone());
//   // let session_store = tower_sessions::diesel_store::DieselStore
//   // let session_store = PostgresStore::new(pool.clone());
//   // let postgres_store = PostgresStore::new(pool);

//   // TODO(TK 2024-02-29): migrate db here

//   let session_store = MemoryStore::default(); // placeholder

//   let session_layer = SessionManagerLayer::new(session_store)
//     .with_secure(false)
//     .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));

// let deletion_task = task::spawn(
//   session_store.clone().continuously_delete_expired(tokio::time::Duration::from_secs(60)),
// );

// let session_layer = SessionManagerLayer::new(session_store)
//   .with_secure(false)
//   .with_expiry(Expiry::OnInactivity(Duration::days(1)));

// session_layer
// }
