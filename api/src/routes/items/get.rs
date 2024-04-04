use super::*;

#[utoipa::path(
  get,
  path = "/items/{id}",
  params( ("id" = String, Path, example = Uuid::new_v4) ),
  responses(
    (status = 422, description = "Invalid id"),
    (status = 404, description = "User not found"),
    (status = 200, description = "Success", body = GetItemResponse),// todo(define reduced UserResponse body)
  ),
  )]
/// Get item.
///
/// If user is authenticated, ...todo
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L92
pub async fn get_item(
  State(state): State<SharedState>,
  Path(id): Path<Uuid>,
  auth_session: AuthSession,
) -> ApiResult<Json<Item>> {
  trace!("get_item called with id: {id}");
  let is_authenticated = auth_session.is_authenticated_and_not_banned();
  // mark show_dead_comments
  // create comments query
  // get item, comments, total comments number
  //
  // let user = db::queries::users::get_item(pool, &)
  //   .await?
  //   .ok_or(ApiError::DbEntryNotFound("that user does not exist".to_string()))?;
  // // todo(auth): currently, we return the whole user.
  // // When auth is implemented, we will want to return different user data, per the caller's
  // auth. info!("found user: {user:?}");
  // Ok(Json(user))
  todo!()
}
