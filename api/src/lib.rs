#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod api;
mod auth;
pub mod error;
#[cfg(test)] mod tests;
mod utils;
mod session;

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
use diesel_async::{
  pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
  AsyncPgConnection,
};
use error::ApiError;
use tracing::info;

use crate::api::comments::comment_router;

pub type DbPool = Pool<AsyncPgConnection>;

/// Access to the database
#[derive(Clone)]
pub struct SharedState {
  pub pool: DbPool,
}

impl Default for SharedState {
  fn default() -> Self {
    let db_url = "postgres://localhost:5432";
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = Pool::builder(config).build().unwrap();
    Self { pool }
  }
}

pub async fn api_router() -> Router {
  let session_store = MemoryStore::default();
  let session_layer = SessionManagerLayer::new(session_store);
  let state = SharedState::default();
  // let auth_layer = AuthManagerLayerBuilder::new(state, session_layer).build();

  Router::new()
    .nest("/comments", comment_router().await)
    // .nest("/auth", auth)
        // .route_layer(login_required!(SharedState, login_url = "/login"))
        // .layer(auth_layer)
        // .with_state(state)
}
