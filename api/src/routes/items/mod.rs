mod payload;
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
use tracing::{debug, info, warn};
use uuid::Uuid;

use super::SharedState;
use crate::auth::AuthSession;
use crate::{
  // auth::{self, assert_authenticated},
  error::ApiError,
  ApiResult,
};

/// Router to be mounted at "/items"
pub fn items_router(state: SharedState) -> Router {
  Router::new()
    .route("/:id", routing::get(get::get_item_simple))
    .route("/", routing::post(post::create_item))
    .with_state(state)
}

pub(super) mod get {
  use super::*;

  #[utoipa::path(
      get,
      path = "/items/{id}",
      params( ("username" = String, Path, example = "alice") ),
      responses(
        // todo(auth) auth error
        // (status = 401, description = "Unauthorized"),
        (status = 422, description = "Invalid id"),
        (status = 404, description = "User not found"),
        (status = 200, description = "Success", body = User),// todo(define reduced UserResponse body)
      ),
  )]
  /// Get item.
  ///
  /// todo(auth): currently, we return the whole item. We actually want to return other stuff.
  ///
  /// ref get_public: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L223
  /// ref get_private: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L244
  pub async fn get_item_simple(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    auth_session: AuthSession, // todo(auth)
  ) -> ApiResult<Json<Item>> {
    debug!("get_item called with id: {id}");
    let pool = &state.pool;
    // username.validate(&())?;
    // let user = db::queries::users::get_item(pool, &)
    //   .await?
    //   .ok_or(ApiError::DbEntryNotFound("that user does not exist".to_string()))?;
    // // todo(auth): currently, we return the whole user.
    // // When auth is implemented, we will want to return different user data, per the caller's
    // auth. info!("found user: {user:?}");
    // Ok(Json(user))
    todo!()
  }
}
pub(super) mod post {
  use super::*;

  #[utoipa::path(
      post,
      path = "/items",
      request_body = ItemPayload,
      params( ("username" = String, Path, example = "alice") ),
      responses(
        // todo(auth) auth error
        (status = 401, description = "Unauthorized"),
        (status = 422, description = "Invalid Payload"),
        (status = 409, description = "Duplication Conflict"),
        (status = 200, description = "Success"), 
      ),
  )]
  pub async fn create_item(
    State(state): State<SharedState>,
    auth_session: AuthSession,
    Json(payload): Json<ItemPayload>,
  ) -> ApiResult<StatusCode> {
    debug!(
      "create_item called with payload: {payload:?} by: {}",
      auth_session.user.as_ref().map(|u| &u.0.username.0).unwrap_or(&"".into())
    );
    // todo(error handling): error passing like this should probably be a defined method for DRY
    let user = auth_session
      .user
      .as_ref()
      .clone()
      .ok_or(ApiError::Unauthorized("must be logged in".to_string()))?;
    if user.0.username != payload.username {
      return Err(ApiError::Unauthorized("must be logged in as the user".to_string()));
    };
    payload.validate(&())?;
    let item: Item = payload.into_item().await;
    db::queries::items::create_item(&state.pool, &item).await?;

    info!("created item: {item:?}");
    Ok(StatusCode::OK)
  }
}
pub(super) mod put {
  use super::*;

  // todo: we should handle the following updates:
  // - upvote, downvote, unvote
  // - favorite, unfavorite
  // - hide, unhide
  #[utoipa::path(
      put,
      path = "/items/upvote/{id}",
      request_body = ItemPayload,
      params( ("id" = Uuid, Path, example = "todo:uuid") ),
      responses(
        (status = 401, description = "Unauthorized"),
        (status = 422, description = "Invalid Payload"),
        (status = 409, description = "Duplication Conflict"),
        (status = 200, description = "Success"), 
      ),
  )]
  pub async fn update_item(
    State(state): State<SharedState>,
    auth_session: AuthSession,
    Path(payload): Path<Uuid>,
  ) -> ApiResult<StatusCode> {
    // assert authenticated
    todo!()
  }
  // OI includes:
  // - get_edit_item_page_data - if the item is editable, return it, else error
  // - edit_item - update title, text, item_category
  // - get_delete_item_page_data - if the item is deletable, return it, else error
  // get ranked items by page
}
pub(super) mod delete {
  use super::*;
  // - delete_item
  pub async fn delete_item(
    State(state): State<SharedState>,
    auth_session: AuthSession,
    Path(payload): Path<Uuid>,
  ) -> ApiResult<StatusCode> {
    // assert authenticated
    // item should not be dead, user should be the owner, delete expration should be unexpired, and
    // there should be no comments
    todo!()
  }
}
