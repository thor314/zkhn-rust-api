use super::*;

/// Edit an item's title, text, type, or category
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L492
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/index.js#L212
#[utoipa::path(
  put,
  path = "/items/edit-item",
  request_body = ItemPayload,
  responses(
    (status = 401, description = "Unauthorized"),
    (status = 403, description = "Forbidden"),
    (status = 403, description = "Forbidden not editable"),
    (status = 422, description = "Invalid Payload"),
    (status = 200, description = "Success"), 
  ),
  )]
pub async fn edit_item(
  State(state): State<SharedState>,
  auth_session: AuthSession,
  Json(payload): Json<EditItemPayload>,
) -> ApiResult<StatusCode> {
  debug!("edit_item called with payload: {payload:?}");
  payload.validate(&())?;
  let item = queries::items::get_assert_item(&state.pool, payload.id).await?;
  item.assert_is_editable(&state.pool).await?;
  let _session_user = auth_session.get_assert_user_from_session_assert_match(&item.username)?;

  // payload.sanitize() // backlog(sanitize) - sanitize title and text
  // if title changed, we may need to change the item type; see routes/utils.js/getitemtype

  queries::items::edit_item(&state.pool, item.id, &payload.title, payload.category, &payload.text)
    .await?;

  // backlog(search) search::editItem(itemId, newItemTitle, newItemText, newItemCategory).await?;

  Ok(StatusCode::OK)
}
