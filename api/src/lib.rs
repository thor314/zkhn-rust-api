#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod api;
mod auth;
pub mod error;
mod session;
#[cfg(test)] mod tests;
mod utils;

use anyhow::Context;
use axum::{
  extract::Request,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use axum_login::{
  login_required,
  tower_sessions::{MemoryStore, SessionManagerLayer},
  AuthManagerLayerBuilder,
};
use db::DbPool;
use error::ApiError;
use tower_sessions::{ExpiredDeletion, Expiry};
use tower_sessions_sqlx_store::PostgresStore;
use tracing::info;

use crate::api::comments::comment_router;
pub use crate::auth::{auth_router, AuthSession};

pub type ApiResult<T> = Result<T, ApiError>;

/// shared state for handlers to access via the State Extractor
#[derive(Clone)]
pub struct SharedState {
  /// Access to the database
  pub pool: DbPool,
}

impl SharedState {
  fn new(pool: DbPool) -> Self { Self { pool } }
}

/// Build the routes and add middleware for:
/// - Session management
/// - Authentication
/// - State
pub async fn api_router(pool: &DbPool) -> ApiResult<Router> {
  let state = SharedState::new(pool.clone());
  let session_layer = get_session_layer(pool).await?;

  let router = Router::new()
    .merge(standard_router(state)) // 
    .layer(session_layer.clone()) // must precede auth router
    .merge(auth_router(pool, &session_layer)) // all routes above this may have auth middleware applied
;

  Ok(router)
}

// todo: might have to move state into here
fn standard_router(state: SharedState) -> Router {
  Router::new().with_state(state).nest("/comments", comment_router())
  // .nest("/users", user_router())
}

// ref: https://github.com/maxcountryman/tower-sessions-stores/blob/main/sqlx-store/examples/postgres.rs
/// Get the `tower-sessions` Manager Layer,
async fn get_session_layer(
  pool: &DbPool,
) -> ApiResult<tower_sessions::SessionManagerLayer<PostgresStore>> {
  let session_store = PostgresStore::new(pool.clone());

  // delete expired connections continuously
  let deletion_task = tokio::task::spawn(
    session_store.clone().continuously_delete_expired(tokio::time::Duration::from_secs(60)),
  );
  // TODO: bug, leave minor optimization commented for now
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
  // todo!()
}
