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
  let user = auth_session.user.ok_or(ApiError::UnauthorizedPleaseLogin)?.0.clone();
  payload.validate(&())?;
  let item: Item = payload.into_item(user.username).await;
  db::queries::items::create_item(&state.pool, &item).await?;

  info!("created item: {item:?}");
  Ok(StatusCode::OK)
}
