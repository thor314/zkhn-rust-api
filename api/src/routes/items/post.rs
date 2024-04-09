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
    // (status = 409, description = "Duplication Conflict"), - cannot occur, uuid generated on server
    (status = 200, body = Uuid),
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
) -> ApiResult<Json<Uuid>> {
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
    // (status = 403, description = "Forbidden"),
    (status = 409, description = "Duplication Conflict"),
    (status = 200, body = Uuid),
  ),
  )]
/// Submit an {up,down,un}vote on an item:
/// - get the user from the session store
/// - get the item from the database, and any previously existing vote on the item
/// - if the user has already voted on the item in the same way: 409 Conflict
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
) -> ApiResult<StatusCode> {
  debug!("vote_item called with payload: {payload:?}");
  let user = auth_session.get_assert_user_from_session()?;
  let (item, vote) = tokio::try_join!(
    queries::items::get_assert_item(&state.pool, payload.content_id),
    queries::user_votes::get_item_vote(&state.pool, &user.username, payload.content_id),
  )?;

  let is_duplication = vote.as_ref().map(|v| v.vote_state == payload.vote_state).unwrap_or(false);
  if is_duplication {
    return Err(ApiError::UniqueViolation(
      "User has already voted identically on this item".into(),
    ));
  }

  queries::user_votes::vote_on_item(state.pool, item.id, user.username, payload.vote_state, vote)
    .await?;

  Ok(StatusCode::OK)
}

#[utoipa::path(
  post,
  path = "/items/favorite",
  request_body = FavoritePayload,
  responses(
    (status = 400, description = "Payload Parsing failed"),
    (status = 401, description = "Unauthorized"),
    (status = 403, description = "Forbidden"),
    (status = 422, description = "Invalid Payload"),
    (status = 409, description = "Duplication Conflict"),
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
) -> ApiResult<()> {
  trace!("favorite_item called with payload: {payload:?}");
  let user = auth_session.get_assert_user_from_session()?;
  let (item, favorite) = tokio::try_join!(
    queries::items::get_assert_item(&state.pool, payload.id),
    queries::user_favorites::get_favorite(&state.pool, &user.username, payload.id),
  )?;

  Ok(())
}

#[utoipa::path(
  post,
  path = "/items/hide",
  request_body = HidePayload,
  responses(
    (status = 400, description = "Payload Parsing failed"),
    (status = 401, description = "Unauthorized"),
    (status = 403, description = "Forbidden"),
    (status = 422, description = "Invalid Payload"),
    (status = 409, description = "Duplication Conflict"),
    (status = 200),
  ),
  )]
/// Submit an [un]hide on an item.
///
/// Return conflict if user has already [un]hidden the item.
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L414
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/index.js#L153
pub async fn hide_item(
  State(state): State<SharedState>,
  auth_session: AuthSession,
  Json(payload): Json<HiddenPayload>,
) -> ApiResult<()> {
  trace!("hide_item called with payload: {payload:?}");
  let user = auth_session.get_assert_user_from_session()?;
  let (item, hide) = tokio::try_join!(
    queries::items::get_assert_item(&state.pool, payload.id),
    queries::hiddens::get_hidden(&state.pool, &user.username, payload.id),
  )?;

  Ok(())
}
