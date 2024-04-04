use tracing::trace;

use super::*;
use crate::auth::AuthenticationExt;

#[utoipa::path(
  post,
  path = "/items",
  request_body = ItemPayload,
  responses(
    (status = 401, description = "Unauthorized"),
    (status = 403, description = "Forbidden"),
    (status = 422, description = "Invalid Payload"),
    (status = 409, description = "Duplication Conflict"),
    (status = 200),
  ),
  )]
/// Create a new item
pub async fn create_item(
  State(state): State<SharedState>,
  auth_session: AuthSession,
  Json(payload): Json<ItemPayload>,
) -> ApiResult<StatusCode> {
  trace!("create_item called with payload: {payload:?}");
  payload.validate(&())?;
  let user = auth_session.get_user_from_session()?;
  let item = payload.into_item(user.username).await;
  db::queries::items::create_item(&state.pool, &item).await?;

  debug!("created item: {item:?}");
  Ok(StatusCode::OK)
}
