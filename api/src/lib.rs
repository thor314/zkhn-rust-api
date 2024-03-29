#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]
#![allow(unused_mut)]

mod auth;
mod cors;
pub mod error;
pub mod routes;
mod sessions;
#[cfg(test)] mod tests;
mod utils;

use auth::get_auth_layer;
use axum::{routing, Router};
use axum_login::{
  login_required,
  tower_sessions::{cookie::time::Duration, session_store, Expiry, SessionManagerLayer},
  AuthManagerLayerBuilder,
};
use db::DbPool;
use error::ApiError;
use routes::routes;
use sessions::get_session_layer;
use tower_sessions::service::SignedCookie;
use tower_sessions_sqlx_store::PostgresStore;
use tracing::info;

use crate::auth::AuthBackend;

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

// // () must implement FromRef<SharedState> for `axum_garde` to be able to validate payloads
// // https://docs.rs/axum_garde/latest/axum_garde/index.html#getting-started
// impl axum::extract::FromRef<SharedState> for () {
//   fn from_ref(_: &SharedState) {}
// }

// /// Build the routes and add middleware for:
// /// - Session management
// /// - Authentication
// /// - State
// pub async fn router(pool: &DbPool, analytics_key: Option<String>) -> ApiResult<Router> {
//   let state = SharedState::new(pool.clone());
//   let auth_layer = {
//     let session_layer = sessions::get_session_layer(pool).await?;
//     let auth_backend = auth::AuthBackend::new(pool.clone());
//     let auth_layer = auth::get_auth_layer(auth_backend.clone(), session_layer.clone());
//     AuthManagerLayerBuilder::new(auth_backend, session_layer).build()
//   };

//   let router = Router::new()
//     .route("/dummy", routing::get(|| async { "dummy route" }))
//     // login protected routes go above the login route_layer
//     .route_layer(login_required!(AuthBackend, login_url = "/login"))
//     // unprotected routes (like "/login") go below the login route_layer
//     .merge(routes(state))
//     // .merge(auth_router()) // todo: move to routes router
//     .layer(auth_layer)
//     .layer(cors::cors_layer())
//     // NOTE: analytics slowing server down dramatically, removal pending
//     // .layer(Analytics::new(analytics_key.unwrap_or("".to_string()))) // must precede auth
// router     //
//     ;

//   Ok(router)
// }

use tower_sessions::{cookie::Key, ExpiredDeletion};

pub async fn app(pool: DbPool) -> ApiResult<Router> {
  let session_layer = create_migrate_session_layer(pool.clone()).await?;
  let auth_layer = get_auth_layer(pool.clone(), session_layer);

  // serve the router and layer any route-agnostic middleware.
  let router = routes::routes(pool)
    // routes::routes(pool, auth_layer) // todo: remove when verified
    .layer(cors::cors_layer())
    .layer(auth_layer);

  Ok(router)
}

pub type MySessionManagerLayer = SessionManagerLayer<PostgresStore, SignedCookie>;

pub async fn create_migrate_session_layer(pool: DbPool) -> ApiResult<MySessionManagerLayer> {
  // create the session store and migrate the database
  let session_store = tower_sessions_sqlx_store::PostgresStore::new(pool);
  session_store.migrate().await?;

  // create a deletion task to continuously delete expired sessions
  // todo(sessions) - how to handle this deletion task to ensure it doesn't drop from scope?
  let deletion_task = tokio::task::spawn(
    session_store.clone().continuously_delete_expired(tokio::time::Duration::from_secs(60)),
  );

  // create the session layer
  let session_layer = SessionManagerLayer::new(session_store)
    .with_secure(false)
    .with_expiry(Expiry::OnInactivity(Duration::days(1)))
    // todo(prod): insecure to generate cryptographic key this way
    .with_signed(Key::generate());

  Ok(session_layer)
}
