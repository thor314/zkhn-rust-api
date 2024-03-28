//! Provided methods:
//! - `create_comment`
//! - `get_comment`
//! - `update_comment_vote`
//! - `update_comment_favorite`
//! - `update_comment_text`
//! - `delete_comment`
//!
//! todo: more
use std::sync::Arc;

use anyhow::Context;
use axum::{
  extract::{Path, State},
  http::StatusCode,
  Json, Router,
};
use axum_login::AuthUser;
use db::{
  models::{
    comment::{self, Comment},
    user_vote::{self, UserVote, VoteState},
  },
  queries, DbError,
};
use futures::{select, FutureExt};
use garde::Validate;
use tokio::spawn;
use uuid::Uuid;

use super::payload::CommentPayload;
use crate::{
  // auth::{self, assert_authenticated, AuthSession},
  error::ApiError,
  ApiResult,
  DbPool,
  SharedState,
};

// /// if user is signed in, check if the user has voted on this comment.
// /// If no comment exists, return Not Found.
// /// If the comment exists, but the user is not signed in, return the Ok((Comment, None)).
// /// If the comment exists, and the user is signed in, return the Ok((Comment, bool)), where bool
// /// indicates whether the user has voted.
// todo: this method diverges significantly from the js api, including not taking page argument
// pub async fn get_comment(
//   State(state): State<SharedState>,
//   Path(comment_id): Path<Uuid>,
//   // auth_session: AuthSession,
// ) -> ApiResult<(Json<Comment>, Json<Option<VoteState>>)> {
//   let comment = queries::get_comment(&state.pool, comment_id)
//     .await?
//     .ok_or(ApiError::DbEntryNotFound("comment not found in db".into()))?;

// match auth_session.user {
//   Some(user) => {
//     let username = &user.username;
//     let user_vote = queries::get_user_vote_by_content_id(&state.pool, &username.0, comment_id)
//       .await
//       .context("no vote found")?;
//     let vote_state = user_vote.map(|v| v.vote_state);
//     Ok((Json(comment), Json(vote_state)))
//   },
//   None => Ok((Json(comment), Json(None))),
// todo!()

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
// }

// /// Add a new comment to the database.
// /// Also update user karma, and item comment count, and tell the search-api.
// pub async fn create_comment(
//   State(state): State<SharedState>,
//   Json(payload): Json<CommentPayload>,
//   // auth_session: AuthSession,
// ) -> ApiResult<StatusCode> {
//   // assert_authenticated(&auth_session)?;
//   // todo: item is dead
//   // assert item exists?
//   let item = queries::get_item(&state.pool, payload.parent_item_id)
//     .await?
//     .ok_or(ApiError::DbEntryNotFound("comment not found in db".into()))?;
//   payload.validate(&())?;
//   let comment = Comment::try_from(payload)?;
//   queries::insert_comment(&state.pool, &comment).await?;

//   Ok(StatusCode::CREATED)
// }

// pub async fn update_comment_vote(
//   State(mut state): State<SharedState>,
//   Path((comment_id, parent_item_id, vote_state)): Path<(Uuid, Uuid, i8)>,
//   // auth_session: AuthSession,
// ) -> ApiResult<StatusCode> {
//   // assert_authenticated(&auth_session)?;
//   let username = &auth_session.user.unwrap().username;

//   let (comment, user_vote) = {
//     let comment_task = queries::get_comment(&state.pool, comment_id);
//     let user_vote_task = queries::get_user_vote_by_content_id(&state.pool, &username.0,
// comment_id);     let (comment_result, maybe_user_vote) = tokio::try_join!(comment_task,
// user_vote_task)?;     let comment = comment_result.context("failed to query queries for
// comment")?;     (comment, maybe_user_vote)
//   };

//   let vote_state = VoteState::from(vote_state);
//   if let Some(user_vote) = user_vote {
//     if user_vote.vote_state == vote_state {
//       // user submitted a vote, but it's the same as the current vote; no-op
//       return Ok(StatusCode::OK);
//     }
//   }

//   // create a new UserVote and increment the comment author's karma
//   queries::submit_comment_vote(
//     &mut state.pool,
//     comment_id,
//     &username.0,
//     parent_item_id,
//     vote_state,
//   )
//   .await?;

//   Ok(StatusCode::OK)
// }

// /// favorite state: 1 to favorite, 0 to unfavorite
// pub async fn update_comment_favorite(
//   State(state): State<SharedState>,
//   Path((comment_id, set_favorite_state)): Path<(Uuid, i8)>,
//   auth_session: AuthSession,
// ) -> ApiResult<StatusCode> {
//   assert_authenticated(&auth_session)?;
//   let username = &auth_session.user.unwrap().username;

//   let (comment, maybe_favorite) = {
//     let comment_task = queries::get_comment(&state.pool, comment_id);
//     let favorite_task =
//       queries::get_user_favorite_by_username_and_item_id(&state.pool, &username.0, comment_id);
//     let (comment_result, maybe_favorite) = tokio::try_join!(comment_task, favorite_task)?;
//     let comment = comment_result.context("failed to query queries for comment")?;
//     (comment, maybe_favorite)
//   };

//   if let Some(ref favorite) = maybe_favorite {
//     if set_favorite_state == 1 {
//       // already favorite, do nothing
//       return Ok(StatusCode::OK);
//     }
//   } else if set_favorite_state == 0 {
//     // already not favorite, do nothing
//     return Ok(StatusCode::OK);
//   } else {
//     return Err(ApiError::DoublySubmittedChange("favorite already submitted".into()));
//   }

//   // update favorite
//   queries::insert_or_delete_user_favorite_for_comment(
//     &state.pool,
//     &username.0,
//     maybe_favorite,
//     comment_id,
//   )
//   .await?;
//   todo!()
// }

// pub async fn update_comment_text(
//   State(state): State<SharedState>,
//   auth_session: AuthSession,
//   body: String,
// ) -> ApiResult<StatusCode> {
//   assert_authenticated(&auth_session)?;
//   let username = &auth_session.user.unwrap().username;
//   todo!()
// }

// pub async fn delete_comment(
//   State(state): State<SharedState>,
//   auth_session: AuthSession,
//   Path(comment_id): Path<Uuid>,
// ) -> ApiResult<StatusCode> {
//   assert_authenticated(&auth_session)?;
//   let username = &auth_session.user.unwrap().username;

//   let item_id = queries::get_comment(&state.pool, comment_id)
//     .await?
//     .ok_or(ApiError::DbEntryNotFound("comment not found in db".into()))?
//     .parent_item_id;
//   queries::delete_comment(&state.pool, comment_id, item_id).await?;

//   Ok(StatusCode::OK)
// }

// // get reply page data
// // get newest comments by page
// // get user comments by page
// // get user favorited comments by page
// // get user upvoted comments by page
// // update all comments to algolia
