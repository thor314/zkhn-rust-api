#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod auth;
pub mod error;
mod routes;
mod sessions;
#[cfg(test)] mod tests;
mod utils;

use anyhow::Context;
use axum::Router;
use axum_analytics::Analytics;
use db::DbPool;
use error::ApiError;
use routes::router_internal;
use sessions::get_session_layer;
use tracing::info;

pub use crate::{
  auth::{auth_router, AuthSession},
  routes::users::payload::UserPayload, // todo: this seems weird
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
