use db::{models::user_favorite::FavoriteStateEnum, Ulid};

use super::*;

#[utoipa::path(
  post,
  path = "/items",
  request_body = CreateItemPayload,
  responses(
    (status = 400, description = "Payload Parsing failed"),
    (status = 401, description = "Unauthorized"),
    (status = 403, description = "ForbiddenBanned"),
    (status = 422, description = "Invalid Payload"),
    // (status = 409, description = "Duplication Conflict"), - cannot occur, ulid generated on server
    (status = 200, body = Ulid),
  ),
  )]
/// Create a new item. The user must be logged in to call this method.
/// - validate payload
/// - create a new item
/// - increment user karma
/// - return the item's id
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L39
pub async fn create_item(
  State(state): State<SharedState>,
  auth_session: AuthSession,
  Json(payload): Json<CreateItemPayload>,
) -> ApiResult<Json<Ulid>> {
  debug!("create_item called with payload: {payload:?}");
  payload.validate(&())?;
  let user = auth_session.get_assert_user_from_session()?;
  let item = payload.into_item(user.username).await;
  queries::items::create_item(&state.pool, &item).await?;

  Ok(Json(item.id))
}

#[utoipa::path(
  post,
  path = "/items/vote",
  request_body = VotePayload,
  responses(
    (status = 400, description = "Payload Parsing failed"),
    (status = 401, description = "Unauthorized"),
    (status = 200, body = Ulid),
  ),
  )]
/// Submit an {up,down,un}vote on an item:
/// - get the user from the session store
/// - get the item from the database, and any previously existing vote on the item
///
/// | State \\ Payload | Up     | Down |
/// | ---------------- | ----   | ---- |
/// | None             | Up     | Down |
/// | Up               | None^1 | Down |
/// | Down             | Down   | None |
///
/// ^1: i.e., with prior state Upvote, submitting an Upvote results in None vote.
///
/// - insert the new vote, replacing any prior vote
/// - update the item's points
/// - update recipient user karma
/// - todo(search) update the search api
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L259
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/index.js#L77
pub async fn vote_item(
  State(state): State<SharedState>,
  auth_session: AuthSession,
  Json(payload): Json<VotePayload>,
) -> ApiResult<Json<VoteState>> {
  debug!("vote_item called with payload: {payload:?}");
  let user = auth_session.get_assert_user_from_session()?;
  let item = queries::items::get_assert_item(&state.pool, &payload.content_id).await?;
  let vote_state =
    queries::user_votes::vote_item(&state.pool, &item.id, &user.username, payload.vote_state)
      .await?;

  Ok(Json(vote_state))
}

#[utoipa::path(
  post,
  path = "/items/favorite",
  request_body = FavoritePayload,
  responses(
    (status = 401, description = "Unauthorized"),
    (status = 403, description = "Forbidden"),
    (status = 422, description = "Invalid Payload"),
    (status = 200),
  ),
  )]
/// Submit an [un]favorite on an item.
///
/// Return conflict if user has already [un]favorited the item.
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L363
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/index.js#L115
pub async fn favorite_item(
  State(state): State<SharedState>,
  auth_session: AuthSession,
  Json(payload): Json<FavoritePayload>,
) -> ApiResult<Json<FavoriteStateEnum>> {
  trace!("favorite_item called with payload: {payload:?}");
  let user = auth_session.get_assert_user_from_session()?;
  let item = queries::items::get_assert_item(&state.pool, &payload.id).await?;

  // post the favorite
  let favorite_state =
    queries::user_favorites::favorite_item(&state.pool, &user.username, &item.id).await?;

  Ok(Json(favorite_state))
}
