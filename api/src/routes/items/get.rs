use std::f32::consts::E;

use axum::extract::Query;
use db::{Page, PasswordHash};
use serde_json::Number;

use super::*;

#[utoipa::path(
  get,
  path = "/items/{id}?page={page}",
  params( ("id" = String, Path, example = Uuid::new_v4),
          ("page" = i32, Query, example = Page::default) ),
  responses( (status = 422, description = "Invalid id"),
             (status = 404, description = "User not found"),
             (status = 200, description = "Success", body = GetItemResponse) ),
  )]
/// Get item.
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L92
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/index.js#L52
pub async fn get_item(
  State(state): State<SharedState>,
  Path(id): Path<Uuid>,
  Query(page): Query<Page>,
  auth_session: AuthSession,
) -> ApiResult<Json<GetItemResponse>> {
  debug!("get_item called with id: {id} and page: {page:?}");
  page.validate(&())?;

  let user = auth_session.get_assert_user_from_session().unwrap_or_else(|_| User::new_logged_out());
  let (item, (comments, total_comments)) = tokio::try_join!(
    db::queries::items::get_assert_item(&state.pool, id),
    db::queries::comments::get_comments_page(&state.pool, id, page, user.show_dead),
  )?;

  let get_item_response = GetItemResponse::new(item, comments, total_comments);
  // backlog(refactor) - matching off janky empty username is mega code smell
  if user.username.0.is_empty() {
    // Unauthenticated user
    Ok(Json(get_item_response))
  } else {
    // Authenticated user
    // get user's itemVotes, itemFavorites, itemHiddens, and commentVotes, and update item_response
    //
    // let (item_votes, item_favorites, item_hiddens, comment_votes) = tokio::try_join!(todo)
    Ok(Json(get_item_response))
  }
}

#[utoipa::path(
  get,
  path = "/items/get-edit-item-page-data",
  params( ("id" = String, Path, example = Uuid::new_v4),
          ("page" = i32, Query, example = Page::default) ),
  responses( (status = 422, description = "Invalid id"),
             (status = 404, description = "User not found"),
             (status = 200, description = "Success", body = GetItemResponse) ),
  )]
/// todo idk
///
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L462
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/index.js#L191
pub async fn get_edit_item_page_data(
  State(state): State<SharedState>,
  Path(id): Path<Uuid>,
  Query(page): Query<Page>,
  auth_session: AuthSession,
) -> ApiResult<Json<GetItemResponse>> {
}