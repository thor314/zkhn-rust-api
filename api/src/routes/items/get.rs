use db::models::user_vote::UserVote;

use super::*;

#[utoipa::path(
  get,
  path = "/items/{id}?page={page}",
  params( ("id" = String, Path, example = Uuid::new_v4),
          ("page" = i32, Query, example = Page::default) ),
  responses( (status = 422, description = "Invalid id"),
             (status = 422, description = "Invalid page"),
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
  params( ("id" = String, Path, example = Uuid::new_v4) ),
  responses( (status = 422, description = "Invalid id"),
             (status = 401, description = "Unauthorized"),
             (status = 403, description = "Forbidden"),
             (status = 404, description = "User not found"),
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
  let item = db::queries::items::get_assert_item(&state.pool, id).await?;
  item.assert_editable(&state.pool).await?;
  let _user = auth_session.get_assert_user_from_session_assert_match(&item.username)?;

  Ok(Json(item.into()))
}

#[utoipa::path(
  get,
  path = "/items/get-items-by-page/{item_kind}?page={page}",
  params( ("item_kind" = ItemKind, Query, example = ItemKind::default), 
          ("page" = i32, Query, example = Page::default) ),
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
        queries::items::get_items_created_after(&state.pool, &start_date, &page, None).await?;
      GetItemsPageResponse::new(items, count, page)
    },
    Some(user) => {
      let item_ids_hidden_by_user =
        queries::hiddens::get_hidden_item_ids_after(&state.pool, &user.username, start_date)
          .await?;
      // backlog(show_dead)
      let (items, count) = queries::items::get_items_created_after(
        &state.pool,
        &start_date,
        &page,
        Some(&item_ids_hidden_by_user),
      )
      .await?;
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
