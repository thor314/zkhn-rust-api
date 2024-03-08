use std::sync::Arc;

use anyhow::Context;
use axum::{
  extract::{Path, State},
  http::StatusCode,
  Json, Router,
};
use axum_login::AuthUser;
use db::{
  error::DbError,
  models::{
    comment::{self, Comment},
    user_vote::{self, UserVote, VoteState},
  },
};
use futures::{select, FutureExt};
use tokio::spawn;
use uuid::Uuid;

use super::payload::CommentPayload;
use crate::{
  auth::{self, assert_authenticated, AuthSession},
  error::{ApiError, RouteError},
  ApiResult, DbPool, SharedState,
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
      let vote_state = user_vote.map(|v| v.vote_state);
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

pub async fn update_comment_vote(
  State(mut state): State<SharedState>,
  Path((comment_id, parent_item_id, vote_state)): Path<(Uuid, Uuid, i8)>,
  auth_session: AuthSession,
) -> Result<StatusCode, ApiError> {
  assert_authenticated(&auth_session)?;
  let user_name = &auth_session.user.unwrap().0.username;

  let (comment, user_vote) = {
    let name = user_name.clone();
    let (pool_1, pool_2) = (state.pool.clone(), state.pool.clone());
    let (comment_task, vote_task) = (
      spawn(async move { db::get_comment_by_id(&pool_1, comment_id).await }),
      spawn(async move {
        db::get_user_vote_by_content_id(&pool_2, &name, comment_id)
          .await
          .context("Error querying user vote")
      }),
    );
    let (comment_result, vote_result) = tokio::try_join!(comment_task, vote_task)?;
    (
      comment_result.context("failed to query db for comment")?.ok_or(RouteError::NotFound)?,
      vote_result.context("failed to query db for vote")?,
    )
  };

  let vote_state = VoteState::from(vote_state);
  if let Some(user_vote) = user_vote {
    if user_vote.vote_state == vote_state {
      // user submitted a vote, but it's the same as the current vote; no-op
      return Ok(StatusCode::OK);
    }
  }

  // create a new UserVote and increment the comment author's karma
  db::submit_comment_vote(&mut state.pool, comment_id, user_name, parent_item_id, vote_state)
    .await?;

  Ok(StatusCode::OK)
}

/// favorite state: 0 to unfavorite, 1 to favorite
pub async fn update_comment_favorite(
  State(mut state): State<SharedState>,
  Path((comment_id, parent_item_id, set_favorite_state)): Path<(Uuid, Uuid, i8)>,
  auth_session: AuthSession,
) -> Result<StatusCode, ApiError> {
  assert_authenticated(&auth_session)?;
  let user_name = &auth_session.user.unwrap().0.username;

  let (comment, maybe_favorite) = {
    let name = user_name.clone();
    let (pool_1, pool_2) = (state.pool.clone(), state.pool.clone());
    let (comment_task, favorite_task) = (
      spawn(async move { db::get_comment_by_id(&pool_1, comment_id).await }),
      spawn(async move {
        db::get_user_favorite_by_username_and_item_id(&pool_2, &name, comment_id)
          .await
          .context("Error querying user vote")
      }),
    );
    let (comment_result, favorite_result) = tokio::try_join!(comment_task, favorite_task)?;
    (
      comment_result.context("failed to query db for comment")?.ok_or(RouteError::NotFound)?,
      favorite_result.context("failed to query db for favorite")?,
    )
  };

  if let Some(favorite) = maybe_favorite {
    if set_favorite_state == 1 {
      // already favorite, do nothing
      return Ok(StatusCode::OK);
    }
  } else if set_favorite_state == 0 {
    // already not favorite, do nothing
    return Ok(StatusCode::OK);
  }

  // update favorite
  // db::get_user_favorite_by_username_and_item_id(&state.pool, user_name, comment_id).await?
  todo!()
}
