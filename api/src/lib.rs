#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]
#![allow(unused_mut)]

mod auth;
mod error;
mod routes;
mod sessions;
mod utils;

use axum::Router;
use db::DbPool;
use tower_cookies::Key;

use self::{auth::get_auth_layer, sessions::create_migrate_session_layer};

pub(crate) type ApiResult<T> = Result<T, ApiError>;

// export payloads and responses
pub use self::{
  error::ApiError,
  routes::{items::*, users::*},
};

pub const MINIMUM_KARMA_TO_DOWNVOTE: i32 = 10; // todo(config)
pub const COMMENTS_PER_PAGE: usize = 10; // todo(config)

pub async fn app(pool: DbPool, session_key: Key) -> ApiResult<Router> {
  let session_layer = create_migrate_session_layer(pool.clone(), session_key).await;
  let auth_layer = get_auth_layer(pool.clone(), session_layer);

  // serve the router and layer any route-agnostic middleware.
  let router = routes::routes(pool).layer(auth_layer);

  Ok(router)
}
