#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod error;
#[cfg(test)] mod tests;
mod utils;

use db::DbPool;
use error::ServerError;
use sqlx::PgPool;
use tower_sessions::{session_store::ExpiredDeletion, Expiry, SessionManagerLayer};
// use tokio::{signal, task::AbortHandle};
use tower_sessions_sqlx_store::PostgresStore;

#[shuttle_runtime::main]
async fn main(
  #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
  #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
  utils::setup(&secret_store).unwrap();
  db::migrate(&pool).await.unwrap();
  let router = api::api_router(pool.clone()).await;
  let session_layer = get_session_layer(&pool).await?;
  let router = router.layer(session_layer);

  Ok(router.into())
}

// ref: https://github.com/maxcountryman/tower-sessions-stores/blob/main/sqlx-store/examples/postgres.rs
/// Get the `tower-sessions` Manager Layer,
async fn get_session_layer(
  pool: &DbPool,
) -> Result<tower_sessions::SessionManagerLayer<PostgresStore>, ServerError> {
  let session_store = PostgresStore::new(pool.clone());

  // delete expired connections continuously
  let deletion_task = tokio::task::spawn(
    session_store.clone().continuously_delete_expired(tokio::time::Duration::from_secs(60)),
  );
  deletion_task.await.map_err(ServerError::from)?.map_err(ServerError::from)?;

  let manager = SessionManagerLayer::new(session_store)
    .with_secure(false) // todo
    .with_expiry(Expiry::OnInactivity(time::Duration::seconds(10)));
  Ok(manager)
}
