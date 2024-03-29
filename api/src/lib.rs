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

use tower_sessions::{cookie::Key, ExpiredDeletion};

pub async fn app(pool: DbPool) -> ApiResult<Router> {
  let session_layer = create_migrate_session_layer(pool.clone()).await?;
  let auth_layer = get_auth_layer(pool.clone(), session_layer);

  // serve the router and layer any route-agnostic middleware.
  let router = routes::routes(pool)
    // routes::routes(pool, auth_layer) // todo: remove when verified
    .layer(cors::cors_layer())
    // todo(analytics)
    // .layer(Analytics::new(analytics_key.unwrap_or("".to_string()))) // must precede auth
    .layer(auth_layer);

  Ok(router)
}

type MySessionManagerLayer = SessionManagerLayer<PostgresStore, SignedCookie>;

async fn create_migrate_session_layer(pool: DbPool) -> ApiResult<MySessionManagerLayer> {
  // create the session store and migrate the database
  let session_store = tower_sessions_sqlx_store::PostgresStore::new(pool);
  session_store.migrate().await?;

  // create a deletion task to continuously delete expired sessions
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
