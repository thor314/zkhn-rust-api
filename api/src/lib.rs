#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod api;
mod auth;
mod comments;
pub mod error;
mod sessions;
#[cfg(test)] mod tests;
mod user_votes;
mod users;
mod utils;

use anyhow::Context;
use api::router_internal;
use axum::{
  extract::Request,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use axum_analytics::Analytics;
use axum_login::{
  login_required,
  tower_sessions::{MemoryStore, SessionManagerLayer},
  AuthManagerLayerBuilder,
};
use comments::comment_router;
use db::DbPool;
use error::ApiError;
use sessions::get_session_layer;
use tower_sessions::{ExpiredDeletion, Expiry};
use tower_sessions_sqlx_store::PostgresStore;
use tracing::info;

pub use crate::{
  auth::{auth_router, AuthSession},
  user_votes::payload::*,
};

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
pub async fn router(pool: &DbPool, analytics_key: Option<String>) -> ApiResult<Router> {
  let state = SharedState::new(pool.clone());
  let session_layer = get_session_layer(pool).await?;

  let router = Router::new()
    .merge(router_internal(state)) // 
    .layer(session_layer.clone()) // must precede auth router
    .layer(Analytics::new(analytics_key.unwrap_or("".to_string()))) // must precede auth router
    .merge(auth_router(pool, &session_layer)) // all routes above this may have auth middleware applied
;

  Ok(router)
}

