pub(super) mod get;
pub(super) mod payload;
pub(super) mod post;
pub(super) mod put;
pub(super) mod response;

use axum::{
  debug_handler,
  extract::{Path, Query, State},
  http::StatusCode,
  routing, Json, Router,
};
use db::{
  models::{
    comment::Comment,
    item::{Item, ItemCategory, ItemType},
    user::User,
    user_vote::VoteState,
  },
  queries, Domain, Page, Text, TextOrUrl, Timestamp, Title, Url, Username,
};
use garde::Validate;
use serde::{Deserialize, Serialize};
use tokio::try_join;
use tracing::{debug, info, trace, warn};
use utoipa::ToSchema;
use uuid::Uuid;

pub use self::{payload::*, response::*};
use super::SharedState;
use crate::{
  auth::{AuthSession, AuthenticationExt},
  error::ApiError,
  ApiResult, COMMENTS_PER_PAGE,
};

/// Router to be mounted at "/items"
pub(super) fn items_router(state: SharedState) -> Router {
  Router::new()
    .route("/:id", routing::get(get::get_item))
    .route("/edit-item", routing::put(put::edit_item))
    .route("/", routing::post(post::create_item))
    .route("/vote", routing::post(post::vote_item))
    .route("/favorite", routing::post(post::favorite_item))
    .route("/get-edit-item-page-data", routing::get(get::get_edit_item_page_data))
    .route("/delete-item", routing::delete(delete::delete_item))
    .route("/get-items-by-page", routing::get(get::get_items_by_page))
    .with_state(state)
}

// todo(score): update scores every 10m
// todo(search): tell algolia things

pub(super) mod delete {
  use super::*;

  /// Delete an item
  ///
  /// https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L559
  /// https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/index.js#L262
  #[utoipa::path(
  put,
  path = "/items/delete-item",
  request_body = ItemPayload,
  responses(
    (status = 401, description = "Unauthorized"),
    (status = 403, description = "Forbidden"),
    (status = 403, description = "Forbidden not editable"),
    (status = 422, description = "Invalid Payload"),
    (status = 200, description = "Success"), 
  ),
  )]
  pub async fn delete_item(
    State(state): State<SharedState>,
    auth_session: AuthSession,
    Path(id): Path<Uuid>,
  ) -> ApiResult<StatusCode> {
    debug!("delete_item called with id: {id:?}");
    let item = db::queries::items::get_assert_item(&state.pool, id).await?;
    item.assert_is_editable(&state.pool).await?;
    let user = auth_session.get_assert_user_from_session_assert_match(&item.username)?;

    // payload.title.sanitize() // backlog(sanitize)
    // backlog(sanitize) item text

    // if title changed, we may need to change the item type; see routes/utils.js/getitemtype
    db::queries::items::delete_item(&state.pool, id, &user.username).await?;

    // backlog(search)
    // await searchApi.deleteItem(itemId, newItemTitle, newItemText, newItemCategory);

    Ok(StatusCode::OK)
  }
}
