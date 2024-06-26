use std::collections::{HashMap, HashSet};

use db::{
  models::{user_favorite::UserFavorite, user_vote::UserVote},
  Ulid,
};

use super::*;

#[utoipa::path(
  get,
  path = "/items/{id}",
  params( ("id" = String, Path, example = Ulid::new),
          Page ),
  responses( (status = 400, description = "Invalid id"),
             (status = 422, description = "Invalid page"),
             (status = 200, description = "Success", body = GetItemResponse) ),
  )]
/// Get item:
/// - validate page and item id
/// - If user is logged out: get and return the item and the `page` of comments
///
/// User is logged in: (todo: blocked by comments upvotes, favorites, and items)
/// - get the user's votes, favorites, and comment votes for the item
/// - validate whether the item may be edited
/// - get the item's comments and update whether they may be edited
/// - and whether they have been upvoted by the user
/// - return the item and comments `page` with the user-specific metadata
///
/// backlog: fetching and recursively updating comment children
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L92
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/index.js#L52
pub async fn get_item(
  State(state): State<SharedState>,
  Path(id): Path<Ulid>,
  Query(page): Query<Page>,
  auth_session: AuthSession,
) -> ApiResult<Json<GetItemResponse>> {
  debug!("get_item called with id: {id} and page: {page:?}");
  page.validate(&())?;

  let session_user = auth_session.get_user_from_session();
  let show_dead = session_user.as_ref().map(|u| u.show_dead).unwrap_or(false);

  let (item, (comments_page, total_comments)) = tokio::try_join!(
    db::queries::items::get_assert_item(&state.pool, &id),
    // todo: concerned about how this fetches a flat, non-recursive comments structure
    db::queries::comments::get_comments_page(&state.pool, &id, page, show_dead),
  )?;

  Ok(Json(match session_user {
    None => GetItemResponse::new(item, comments_page, total_comments, None, None, None)?,
    Some(user) => {
      // get the user-related item-votes, favorites, and comment-votes for this item
      let (vote, favorite, user_comment_votes): (
        Option<UserVote>,
        Option<UserFavorite>,
        Vec<UserVote>,
      ) = tokio::try_join!(
        queries::user_votes::get_item_vote(&state.pool, &user.username, &item.id),
        queries::user_favorites::get_favorite(&state.pool, &user.username, &item.id),
        queries::user_votes::get_user_related_votes_for_item(&state.pool, &user.username, &item.id),
      )?;

      // create the user-related item metadata from the obtained item-related data
      let item_metadata =
        GetItemResponseAuthenticated::new(&state.pool, &item, &vote, &favorite, &user).await;

      // compute the item response from the item, comments, and user-related item metadata
      GetItemResponse::new(
        item,
        comments_page,
        total_comments,
        Some(item_metadata),
        Some(user),
        Some(user_comment_votes),
      )?
    },
  }))
}

#[utoipa::path(
  get,
  path = "/items/get-items-by-page/{item_kind}",
  params( ("item_kind" = ItemKind, Path, example = ItemKind::default), 
          Page ),
  responses(
             (status = 400, description = "Invalid page"),
             (status = 401, description = "Unauthorized"),
             (status = 403, description = "Forbidden"),
             (status = 404, description = "User not found"),
             (status = 200, description = "Success", body = GetItemsPageResponse) ),
  )]
/// Get items by page.
/// todo: only `ranked` is currently implemented for ItemKind.
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L611
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/index.js#L282
pub async fn get_items_by_page(
  State(state): State<SharedState>,
  Path(item_kind): Path<ItemKind>,
  Query(page): Query<Page>,
  auth_session: AuthSession,
) -> ApiResult<Json<GetItemsPageResponse>> {
  debug!("get_items_by_page with page: {page:?} and kind: {item_kind:?}");
  let start_date = Timestamp(chrono::Utc::now() - chrono::Duration::try_hours(48).unwrap());
  let session_user = auth_session.get_user_from_session();
  let (items, count) =
    queries::items::get_items_created_after(&state.pool, &start_date, &page).await?;

  Ok(Json(match session_user {
    None => {
      // not logged in: just return the page of items and the total number of items
      GetItemsPageResponse::new(items, count, page, HashMap::new(), None)
    },
    Some(user) => {
      // backlog(show_dead)
      // user is logged in:
      // - retreive the user's votes (if any) on the items in the retreived page
      // - for each item, annotate, whether the item may be edit/deleted, and whether the user has
      //   voted on the item
      let item_ids = items.iter().map(|item| item.id.to_string()).collect::<Vec<_>>();
      let item_votes =
        queries::user_votes::get_votes_matching_ids(&state.pool, &user.username, &item_ids).await?;
      GetItemsPageResponse::new(items, count, page, item_votes, Some(&user.username))
    },
  }))
}
