#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]
#![allow(unused_mut)]

mod auth;
pub mod error;
mod routes;
mod sessions;
mod utils;

use anyhow::Context;
use auth::auth_router;
pub use auth::credentials::{oauth_creds::OAuthCreds, password_creds::PasswordCreds, Credentials};
use axum::{routing, Router};
use axum_analytics::Analytics;
use axum_login::{login_required, AuthManagerLayerBuilder};
use db::DbPool;
use error::ApiError;
use routes::routes;
use sessions::get_session_layer;
use tracing::info;

use crate::auth::AuthBackend;
pub use crate::routes::users::payload::*;

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

// () must implement FromRef<SharedState> for `axum_garde` to be able to validate payloads
// https://docs.rs/axum_garde/latest/axum_garde/index.html#getting-started
impl axum::extract::FromRef<SharedState> for () {
  fn from_ref(_: &SharedState) {}
}

/// Build the routes and add middleware for:
/// - Session management
/// - Authentication
/// - State
pub async fn router(pool: &DbPool, analytics_key: Option<String>) -> ApiResult<Router> {
  let state = SharedState::new(pool.clone());
  let auth_layer = {
    let session_layer = get_session_layer(pool).await?;
    let auth_backend = auth::backend::AuthBackend::new_with_default_client(pool.clone());
    AuthManagerLayerBuilder::new(auth_backend, session_layer).build()
  };

  let router = Router::new()
    .route("/dummy", routing::get(|| async { "dummy route" }))
    // login protected routes go above the login route_layer
    .route_layer(login_required!(AuthBackend, login_url = "/login"))
    // unprotected routes (like "/login") go below the login route_layer
    .merge(routes(state))
    .merge(auth_router()) 
    .layer(auth_layer)
    .layer(Analytics::new(analytics_key.unwrap_or("".to_string()))) // must precede auth router
    //
    ;

  Ok(router)
}
