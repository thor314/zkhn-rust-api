use super::*;

#[utoipa::path(
  get,
  path = "/items/{id}",
  params( ("id" = String, Path, example = Uuid::new_v4) ),
  responses(
    (status = 422, description = "Invalid id"),
    (status = 404, description = "User not found"),
    (status = 200, description = "Success", body = GetItemResponse),// todo(define reduced UserResponse body)
  ),
  )]
/// Get item.
///
/// If user is authenticated, ...todo
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/items/api.js#L92
pub async fn get_item(
  State(state): State<SharedState>,
  Path(id): Path<Uuid>,
  auth_session: AuthSession,
) -> ApiResult<Json<Item>> {
  trace!("get_item called with id: {id}");
  let is_authenticated = auth_session.is_authenticated_and_not_banned();
    // Extract the `show_dead_comments` value from `auth_user`
    let show_dead_comments = todo!("Extract the value from `auth_user.show_dead`");

     // Prepare the `comments_db_query` based on `show_dead_comments`
    let mut comments_db_query = todo!("Create a struct or HashMap for `comments_db_query`");
    comments_db_query.parent_item_id = id;
    comments_db_query.is_parent = true;
    if !show_dead_comments {
        comments_db_query.dead = false;
    }   

    // Fetch the item, comments, and total number of comments concurrently
    let (item, comments, total_num_of_comments) = tokio::join!(
        todo!("Fetch the item using `ItemModel.find_one`"),
        todo!("Fetch the comments using `CommentModel.find` with pagination and sorting"),
        todo!("Count the total number of comments using `CommentModel.count_documents`"),
    );

    // Check if the item exists
    let item = match item {
        Some(item) => item,
        None => return Err(ApiError::NotFound),
    };

    // If the user is not signed in
    if !is_authenticated {
        return Ok(Json(Item {
            success: true,
            item,
            comments,
            is_more_comments: total_num_of_comments > todo!("Calculate the value"),
        }));
    }

    // If the user is signed in
    let (vote_doc, favorite_doc, hidden_doc, comment_vote_docs) = tokio::join!(
        todo!("Fetch the vote document using `UserVoteModel.find_one`"),
        todo!("Fetch the favorite document using `UserFavoriteModel.find_one`"),
        todo!("Fetch the hidden document using `UserHiddenModel.find_one`"),
        todo!("Fetch the comment vote documents using `UserVoteModel.find`"),
    );

    // Set up all item and user relations
    item.voted_on_by_user = vote_doc.is_some();
    item.unvote_expired = todo!("Calculate the unvote expiration based on `vote_doc` and `config.hrs_until_unvote_expires`");
    item.favorited_by_user = favorite_doc.is_some();
    item.hidden_by_user = hidden_doc.is_some();

    // Check if the item is still able to be edited/deleted
    if item.by == auth_session.username {
        let has_edit_and_delete_expired = todo!("Calculate the edit and delete expiration based on `item.created`, `config.hrs_until_edit_and_delete_expires`, and `item.comment_count`");
        item.edit_and_delete_expired = has_edit_and_delete_expired;
    }

    // Prepare to get item comments
    let user_comment_votes: Vec<_> = comment_vote_docs.iter().map(|doc| doc.id).collect();

    // Update each comment recursively
    fn update_comment(comment: &mut Comment, auth_session: &AuthSession, user_comment_votes: &[Uuid], comment_vote_docs: &[CommentVoteDoc]) {
        // TODO: Implement the `update_comment` function
    }

    // Call the `update_comment` function for each comment
    for comment in &mut comments {
        update_comment(comment, &auth_session, &user_comment_votes, &comment_vote_docs);
    }

    Ok(Json(Item {
        success: true,
        item,
        comments,
        is_more_comments: total_num_of_comments > todo!("Calculate the value"),
    }))


  todo!()
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