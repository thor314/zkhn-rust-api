mod delete;
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
    .route("/edit-item", routing::put(put::edit_item))
    .route("/", routing::post(post::create_item))
    .route("/vote", routing::post(post::vote_item))
    .route("/favorite", routing::post(post::favorite_item))
    .route("/hide", routing::post(post::hide_item))
    .route("/get-edit-item-page-data", routing::get(get::get_edit_item_page_data))
    .route("/get-delete-item-page-data", routing::get(get::get_delete_item_page_data))
    .with_state(state)
}
