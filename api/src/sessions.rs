use db::DbPool;
  use tower_sessions::{Expiry, SessionManagerLayer};
  use tower_sessions_sqlx_store::PostgresStore;

  use crate::ApiResult;

  // ref: https://github.com/maxcountryman/tower-sessions-stores/blob/main/sqlx-store/examples/postgres.rs
  /// Get the `tower-sessions` Manager Layer
  pub(crate) async fn get_session_layer(
pool: &DbPool,
  ) -> ApiResult<tower_sessions::SessionManagerLayer<PostgresStore>> {
let session_store = PostgresStore::new(pool.clone());

// TODO: bug, leave minor optimization commented for now
// delete expired connections continuously
// let deletion_task = tokio::task::spawn(
//   session_store.clone().continuously_delete_expired(tokio::time::Duration::from_secs(60)),
// );
// deletion_task
//   .await
//   .map_err(ApiError::from)
//   .context("bad delete")?
//   .map_err(ApiError::from)
//   .context("bad join")?;

let manager = SessionManagerLayer::new(session_store)
.with_secure(false) // todo
.with_expiry(Expiry::OnInactivity(tower_sessions::cookie::time::Duration::seconds(10)));
Ok(manager)
  }
