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
use tracing::info;

use crate::api::comments::comment_router;
// pub use crate::auth::{AuthSession, }
pub use auth::auth_router;

/// Access to the database
#[derive(Clone)]
pub struct SharedState {
  pub pool: DbPool,
}

impl SharedState {
  fn new(pool: DbPool) -> Self { Self { pool } }
}

pub async fn api_router(pool: DbPool) -> Router {
  // let session_store = MemoryStore::default();
  // let session_layer = SessionManagerLayer::new(session_store);
  let state = SharedState::new(pool);
  // let auth_layer = AuthManagerLayerBuilder::new(state, session_layer).build();

  Router::new().with_state(state).nest("/comments", comment_router().await)
  // .nest("/auth", auth)
  // .route_layer(login_required!(SharedState, login_url = "/login"))
  // .layer(auth_layer)
}
