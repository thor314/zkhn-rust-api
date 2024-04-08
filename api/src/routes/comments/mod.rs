mod payload;
mod response;

use std::sync::Arc;

use anyhow::Context;
use axum::{
  extract::{Path, State},
  http::StatusCode,
  routing, Json, Router,
};
use axum_login::AuthUser;
use db::{
  models::{
    comment::{self, Comment},
    user_vote::{self, UserVote, VoteState},
  },
  queries, CommentText, DbError, Page, Title, Username,
};
use futures::{select, FutureExt};
use garde::Validate;
use serde::{Deserialize, Serialize};
use tokio::spawn;
use tracing::debug;
use utoipa::ToSchema;
use uuid::Uuid;

pub use self::{payload::*, response::*};
use super::SharedState;
use crate::{
  auth::{AuthSession, AuthenticationExt},
  error::ApiError,
  ApiResult, CreateItemPayload, DbPool,
};

/// Router to be mounted at "/items"
pub fn comments_router(state: SharedState) -> Router {
  Router::new().route("/", routing::post(post::create_comment)).with_state(state)
}

pub(super) mod post {
  use super::*;

  #[utoipa::path(
  post,
  path = "/",
  request_body = CreateCommentPayload,
  responses(
    (status = 401, description = "Unauthorized"),
    (status = 403, description = "Forbidden"),
    (status = 422, description = "Invalid Payload"),
    (status = 409, description = "Duplication Conflict"),
    (status = 200, body = Uuid),
  ),
  )]
  /// Create a new comment
  ///
  /// https://github.com/thor314/zkhn/blob/main/rest-api/routes/comments/api.js#L18
  pub async fn create_comment(
    State(state): State<SharedState>,
    auth_session: AuthSession,
    Json(payload): Json<CreateCommentPayload>,
  ) -> ApiResult<Json<Uuid>> {
    debug!("create_comment called with payload: {payload:?}");
    payload.validate(&())?;
    let user = auth_session.get_assert_user_from_session()?;
    let item = queries::items::get_assert_item(&state.pool, payload.parent_item_id).await?;
    // todo: commentdata.text sanitize
    let comment = payload.into_comment();
    // todo: increment commenter karma
    queries::comments::create_comment(&state.pool, &comment).await?;
    // removed: update item comment count & parent comment count
    Ok(Json(comment.id))
  }

  pub async fn upvote_comment(
    State(state): State<SharedState>,
    auth_session: AuthSession,
    Path((comment_id, parent_item_id)): Path<(Uuid, Uuid)>,
  ) -> ApiResult<StatusCode> {
    let username = auth_session.get_assert_user_from_session()?.username;
    // let vote_state = VoteState::Upvote;
    // let user_vote = queries::user_votes::get_comment_vote(&state.pool, &username, comment_id).await?;
    // if let Some(user_vote) = user_vote {
    //   if user_vote.vote_state == vote_state {
    //     return Ok(StatusCode::OK);
    //   }
    // }
    // queries::user_votes::submit_comment_vote(&state.pool, comment_id, &username, parent_item_id, vote_state).await?;
    // Ok(StatusCode::OK)
    todo!()
  }
}

pub(super) mod get {
  use super::*;

  #[utoipa::path(
  get,
  path = "/{comment_id}",
  responses(
    (status = 404, description = "Comment not found"),
    (status = 200, description = "Success", body = Comment),
  ),
  )]
  /// Get a comment
  ///
  /// https://github.com/thor314/zkhn/blob/main/rest-api/routes/comments/api.js#L94
  pub async fn get_comment(
    State(state): State<SharedState>,
    Path(comment_id): Path<Uuid>,
    auth_session: AuthSession,
  ) -> ApiResult<Json<GetCommentResponse>> {
    debug!("get_comment called with comment_id: {comment_id:?}");
    let comment = queries::comments::get_assert_comment(&state.pool, comment_id).await?;
    let user = auth_session.get_user_from_session();
    //     comment.pageMetadataTitle = comment.text.replace(/<[^>]+>/g, ""); todo
    // todo: get sorted comment children by points then created
    // todo get total number of children comments
    // todo slice comments per page
    // if user not signed in, return the comment
    // todo: more if the user is signed in

    // Ok(Json(comment))
    todo!()
  }
}

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
// // todo
// pub fn comments_router(state: SharedState) -> Router { Router::new().with_state(state) }

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
