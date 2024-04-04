mod get;
mod payload;
mod post;
mod put;
mod response;

use anyhow::anyhow;
use axum::{
  debug_handler,
  extract::{Path, State},
  http::StatusCode,
  routing, Json, Router,
};
use db::{
  models::{item::Item, user::User},
  AuthToken, DbError, Username,
};
use garde::Validate;
pub use payload::*;
pub use response::*;
use tracing::{debug, info, trace, warn};
use uuid::Uuid;

use super::SharedState;
use crate::{
  auth::{AuthSession, AuthenticationExt},
  error::ApiError,
  ApiResult,
};

/// Router to be mounted at "/items"
pub fn items_router(state: SharedState) -> Router {
  Router::new()
    .route("/:id", routing::get(get::get_item))
    .route("/", routing::post(post::create_item))
    .with_state(state)
}
