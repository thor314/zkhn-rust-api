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
    .route("/get-items-by-page/:item_kind", routing::get(get::get_items_by_page))
    .route("/", routing::post(post::create_item))
    .route("/vote", routing::post(post::vote_item))
    .route("/favorite", routing::post(post::favorite_item))
    .route("/edit-item", routing::put(put::edit_item))
    .route("/delete-item/:id", routing::delete(delete::delete_item))
    .with_state(state)
}

// todo(score): update scores every 10m
// todo(search): tell algolia things

pub(super) mod delete {
  use db::Ulid;

  use super::*;

  /// Delete an item
  ///
  /// https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L559
  /// https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/index.js#L262
  #[utoipa::path(
  delete,
  path = "/items/delete-item/{id}",
  params( ("id" = String, Path, example = Ulid::new) ),
  responses(
    (status = 422, description = "Invalid Payload"),
    (status = 401, description = "Unauthorized"),
    (status = 404, description = "Item not found"),
    (status = 403, description = "Forbidden"),
    (status = 403, description = "Forbidden not editable"),
    (status = 200, description = "Success"), 
  ),
  )]
  pub async fn delete_item(
    State(state): State<SharedState>,
    auth_session: AuthSession,
    Path(id): Path<Ulid>,
  ) -> ApiResult<StatusCode> {
    debug!("delete_item called with id: {id:?}");
    let item = db::queries::items::get_assert_item(&state.pool, &id).await?;
    let user = auth_session.get_assert_user_from_session_assert_match(&item.username)?;
    item.assert_is_editable(&state.pool).await?;
    db::queries::items::delete_item(&state.pool, &item, &user.username).await?;

    // backlog(search)
    // await searchApi.deleteItem(itemId, newItemTitle, newItemText, newItemCategory);

    Ok(StatusCode::OK)
  }
}
