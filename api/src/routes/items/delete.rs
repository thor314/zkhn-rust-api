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
  let session_user = auth_session.get_assert_user_from_session()?;
  let item = db::queries::items::get_assert_item(&state.pool, id).await?;
  // backlog: assert item is editable
  // backlog: assert no comments

  // payload.title.sanitize() // backlog(sanitize)
  // backlog(sanitize) item text
  // backlog validate url

  // if title changed, we may need to change the item type; see routes/utils.js/getitemtype

  // backlog(search)
  // await searchApi.deleteItem(itemId, newItemTitle, newItemText, newItemCategory);

  Ok(StatusCode::OK)
}
