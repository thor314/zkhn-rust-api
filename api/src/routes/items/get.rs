use std::f32::consts::E;

use axum::extract::Query;
use db::{Page, PasswordHash};
use serde_json::Number;

use super::*;

#[utoipa::path(
  get,
  path = "/items/{id}",
  params( ("id" = String, Path, example = Uuid::new_v4),
          ("page" = i32, Query, example = Page::default) ),
  responses( (status = 422, description = "Invalid id"),
             (status = 404, description = "User not found"),
             (status = 200, description = "Success", body = GetItemResponse) ),
  )]
/// Get item.
///
/// If user is authenticated, ...todo
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L92
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/index.js#L52
pub async fn get_item(
  State(state): State<SharedState>,
  Path(id): Path<Uuid>,
  Query(page): Query<Page>,
  auth_session: AuthSession,
) -> ApiResult<Json<GetItemResponse>> {
  trace!("get_item called with id: {id} and page: {page:?}");
  page.validate(&())?;

  let user = auth_session.get_user_from_session().unwrap_or_else(|_| User::new_logged_out());
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
