use db::models::{user_favorite::UserFavorite, user_vote::UserVote};

use super::*;

#[utoipa::path(
  get,
  path = "/items/{id}",
  params( ("id" = String, Path, example = Uuid::new_v4),
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
  Path(id): Path<Uuid>,
  Query(page): Query<Page>,
  auth_session: AuthSession,
) -> ApiResult<Json<GetItemResponse>> {
  debug!("get_item called with id: {id} and page: {page:?}");
  page.validate(&())?;

  let session_user = auth_session.get_user_from_session();
  let show_dead = session_user.as_ref().map(|u| u.show_dead).unwrap_or(false);

  let (item, (comments_page, total_comments)) = tokio::try_join!(
    db::queries::items::get_assert_item(&state.pool, id),
    // todo: concerned about how this fetches a flat, non-recursive comments structure
    db::queries::comments::get_comments_page(&state.pool, id, page, show_dead),
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
        queries::user_votes::get_item_vote(&state.pool, &user.username, item.id),
        queries::user_favorites::get_favorite(&state.pool, &user.username, item.id),
        queries::user_votes::get_user_related_votes_for_item(&state.pool, &user.username, item.id),
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
  path = "/items/get-edit-item-page-data",
  params( ("id" = String, Path, example = Uuid::new_v4) ),
  responses( (status = 422, description = "Invalid id"),
             (status = 401, description = "Unauthorized"),
             (status = 403, description = "Forbidden"),
             (status = 200, description = "Success", body = GetEditItemResponse) ),
  )]
/// Get item content for for editing. User must be authenticated.
///
/// Note that get-delete-item-page-data also maps to this route.
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L462
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/index.js#L191
pub async fn get_edit_item_page_data(
  State(state): State<SharedState>,
  Path(id): Path<Uuid>,
  auth_session: AuthSession,
) -> ApiResult<Json<GetEditItemResponse>> {
  debug!("get_edit_item called with id: {id}");
  let item = queries::items::get_assert_item(&state.pool, id).await?;
  item.assert_is_editable(&state.pool).await?;
  let session_user = auth_session.get_assert_user_from_session_assert_match(&item.username)?;
  // todo(sanitize): item.text -> text_for_editing

  Ok(Json(GetEditItemResponse::new(item, Some(session_user))))
}

#[utoipa::path(
  get,
  path = "/items/get-items-by-page/{item_kind}",
  params( ("item_kind" = ItemKind, Query, example = ItemKind::default), 
          Page ),
  responses( (status = 401, description = "Unauthorized"),
             (status = 403, description = "Forbidden"),
             (status = 404, description = "User not found"),
             (status = 200, description = "Success") ), // response body todo
  )]
/// Get items by page, sorted by SortKind
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
  let start_date = Timestamp(chrono::Utc::now() - chrono::Duration::try_hours(24).unwrap());
  let session_user = auth_session.get_user_from_session();

  Ok(Json(match session_user {
    None => {
      let (items, count) =
        queries::items::get_items_created_after(&state.pool, &start_date, &page).await?;
      GetItemsPageResponse::new(items, count, page)
    },
    Some(user) => {
      // backlog(show_dead)
      let (items, count) =
        queries::items::get_items_created_after(&state.pool, &start_date, &page).await?;
      let user_votes: Vec<UserVote> = queries::user_votes::get_user_votes_on_items_after(
        &state.pool,
        &user.username,
        start_date,
        page,
      )
      .await?;

      // todo: is item allowed to be edited or deleted?

      // todo!()
      GetItemsPageResponse::new(items, count, page)
    },
  }))
}
