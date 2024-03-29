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

use axum::Router;
use db::DbPool;

use self::{
  auth::get_auth_layer, error::ApiError, routes::routes, sessions::create_migrate_session_layer,
};

pub type ApiResult<T> = Result<T, ApiError>;

pub async fn app(pool: DbPool) -> ApiResult<Router> {
  let session_layer = create_migrate_session_layer(pool.clone()).await?;
  let auth_layer = get_auth_layer(pool.clone(), session_layer);

  // serve the router and layer any route-agnostic middleware.
  let router = routes::routes(pool)
    // routes::routes(pool, auth_layer) // todo: remove when verified
    // todo(refactor): cors and analytics could live in server instead
    .layer(cors::cors_layer()) 
    // todo(analytics)
    // .layer(Analytics::new(analytics_key.unwrap_or("".to_string()))) // must precede auth
    .layer(auth_layer);

  Ok(router)
}
