use anyhow::Context;
use axum::{
  extract::{Path, State},
  http::StatusCode,
  Json, Router,
};
use axum_login::AuthUser;
use db::models::{comment::Comment, user_vote::UserVote};
use uuid::Uuid;

use super::payload::CommentPayload;
use crate::{
  auth::{self, assert_authenticated, AuthSession},
  error::{ApiError, RouteError},
  ApiResult, DbPool, SharedState, VoteState,
};

/// Add a new comment to the database.
/// Also update user karma, and item comment count, and tell the search-api.
pub async fn add_new_comment(
  State(state): State<SharedState>,
  Json(payload): Json<CommentPayload>,
  auth_session: AuthSession,
) -> ApiResult<StatusCode> {
  assert_authenticated(&auth_session)?;
  let item =
    db::get_item_by_id(&state.pool, payload.parent_item_id).await?.ok_or(RouteError::NotFound)?;
  let new_comment: Comment = payload.try_into()?;
  db::insert_comment(&state.pool, &new_comment).await?;

  Ok(StatusCode::CREATED)
}

///  if user is signed in, check if the user has voted on this comment.
/// If no comment exists, return Not Found.
/// If the comment exists, but the user is not signed in, return the Ok((Comment, None)).
/// If the comment exists, and the user is signed in, return the Ok((Comment, bool)), where bool
/// indicates whether the user has voted.
// todo: this method diverges significantly from the js api, including not taking page argument
pub async fn get_comment_by_id(
  State(state): State<SharedState>,
  Path(comment_id): Path<Uuid>,
  auth_session: AuthSession,
) -> ApiResult<(Json<Comment>, Json<Option<VoteState>>)> {
  let comment =
    db::get_comment_by_id(&state.pool, comment_id).await?.ok_or(RouteError::NotFound)?;

  match auth_session.user {
    Some(user) => {
      let user_name = &user.0.username;
      let user_vote = db::get_user_vote_by_content_id(&state.pool, user_name, comment_id)
        .await
        .context("no vote found")?;
      let vote_state = user_vote.map(|v| VoteState::from(v));
      Ok((Json(comment), Json(vote_state)))
    },
    None => Ok((Json(comment), Json(None))),
  }

  // todo: the js api contains many more things, that appear to not truly belong in a method such as
  // this

  // // todo: this is not a field on the comment, so it stays commented
  // // comment.pageMetadataTitle = comment.text.replace(/<[^>]+>/g, "");

  // // todo: the js api now sorts comments by points and chunks by page
  // // let mut comments: Vec<Comment> = todo!();
  // // // sort comments first by most points, then by latest date created
  // // comments.sort_by(|a, b| a.points.cmp(&b.points).then_with(||
  // a.created.0.cmp(&b.created.0)));

  // // if (!authUser.userSignedIn) {
  // //   return { success: true, comment: comment };
  // Ok((Json(comment), user_vote))
}

pub async fn upvote_comment(
  State(mut state): State<SharedState>,
  Path((comment_id, parent_item_id)): Path<(Uuid, Uuid)>,
  auth_session: AuthSession,
) -> Result<StatusCode, ApiError> {
  assert_authenticated(&auth_session)?;
  let user_name = &auth_session.user.as_ref().unwrap().0.username;

  let comment =
    db::get_comment_by_id(&mut state.pool, comment_id).await?.ok_or(RouteError::NotFound)?;

  // Query the database for a user vote with the given username and comment id.
  let user_vote = db::get_user_vote_by_content_id(&mut state.pool, user_name, comment_id)
    .await
    .context("Error querying user vote")?;

  // check that the user_vote is not an upvote
  if let Some(vote) = user_vote {
    if vote.upvote {
      return Err(RouteError::BadRequest.into());
    }
  }

  // create a new UserVote and increment the comment author's karma
  db::upvote_comment(&mut state.pool, comment_id, user_name, parent_item_id).await?;

  Ok(StatusCode::OK)
}
