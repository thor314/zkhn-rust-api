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
