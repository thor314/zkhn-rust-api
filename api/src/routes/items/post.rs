use super::*;

#[utoipa::path(
  post,
  path = "/items",
  request_body = CreateItemPayload,
  responses(
    (status = 401, description = "Unauthorized"),
    (status = 403, description = "Forbidden"),
    (status = 422, description = "Invalid Payload"),
    (status = 409, description = "Duplication Conflict"),
    (status = 200, body = Uuid),
  ),
  )]
/// Create a new item
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L39
pub async fn create_item(
  State(state): State<SharedState>,
  auth_session: AuthSession,
  Json(payload): Json<CreateItemPayload>,
) -> ApiResult<Json<Uuid>> {
  trace!("create_item called with payload: {payload:?}");
  payload.validate(&())?;
  let user = auth_session.get_assert_user_from_session()?;
  let item = payload.into_item(user.username).await;
  db::queries::items::create_item(&state.pool, &item).await?;

  debug!("created item: {item:?}");
  Ok(Json(item.id))
}

#[utoipa::path(
  post,
  path = "/items/vote",
  request_body = VotePayload,
  responses(
    (status = 401, description = "Unauthorized"),
    (status = 403, description = "Forbidden"),
    (status = 422, description = "Invalid Payload"),
    (status = 409, description = "Duplication Conflict"),
    (status = 200, body = Uuid),
  ),
  )]
/// Submit an {up,down,un}vote on an item.
///
/// Return Conflict if the user has already voted identically on the item.
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L259
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/index.js#L77
pub async fn vote_item(
  State(state): State<SharedState>,
  auth_session: AuthSession,
  Json(payload): Json<VotePayload>,
) -> ApiResult<Json<Uuid>> {
  trace!("vote_item called with payload: {payload:?}");
  let user = auth_session.get_assert_user_from_session()?;
  let (item, vote) = tokio::try_join!(
    db::queries::items::get_assert_item(&state.pool, payload.id),
    db::queries::user_votes::get_vote(&state.pool, &user.username, payload.id),
  )?;
  // note - diverge from reference, allow submitting user to vote on their own item
  // if vote.matches(&payload}) { return Err(ApiError::Conflict); }

  // todo(vote): insert the new vote
  // let vote = db::models::user_votes::UserVote::new(&user.username, &payload);
  // db::queries::user_votes::insert_vote(&state.pool, &vote);
  // todo(item points)
  // todo(karma)

  debug!("vote submitted");
  Ok(Json(item.id))
}
