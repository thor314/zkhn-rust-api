use super::*;

// todo: we should handle the following updates:
// - upvote, downvote, unvote
// - favorite, unfavorite
// - hide, unhide
#[utoipa::path(
  put,
  path = "/items/upvote/{id}",
  request_body = ItemPayload,
  params( ("id" = Uuid, Path, example = "todo:uuid") ),
  responses(
    (status = 401, description = "Unauthorized"),
    (status = 422, description = "Invalid Payload"),
    (status = 409, description = "Duplication Conflict"),
    (status = 200, description = "Success"), 
  ),
  )]
pub async fn update_item(
  State(state): State<SharedState>,
  auth_session: AuthSession,
  Path(payload): Path<Uuid>,
) -> ApiResult<StatusCode> {
  // assert authenticated
  todo!()
}
// OI includes:
// - get_edit_item_page_data - if the item is editable, return it, else error
// - edit_item - update title, text, item_category
// - get_delete_item_page_data - if the item is deletable, return it, else error
// get ranked items by page
