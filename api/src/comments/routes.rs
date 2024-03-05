use axum::{
  extract::{Path, State},
  http::StatusCode,
  Json, Router,
};
use axum_login::AuthUser;
use db::models::comment::Comment;
use uuid::Uuid;

use super::payload::CommentPayload;
// use sqlx::types::Uuid;
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
pub async fn get_comment_by_id(
  State(state): State<SharedState>,
  Path(comment_id): Path<Uuid>,
  auth_session: AuthSession,
) -> ApiResult<(Json<Comment>, Option<Json<VoteState>>)> {
  let comment =
    db::get_comment_by_id(&state.pool, comment_id).await?.ok_or(RouteError::NotFound)?;
  let user_vote = auth_session.user.map(|user| async move {
    let user_name = &user.0.username;
    db::get_user_vote_by_content_id(&state.pool, user_name, comment_id).await.ok()?
  });
  // todo: should I mirror this sanitization?
  // comment.pageMetadataTitle = comment.text.replace(/<[^>]+>/g, "");

  todo!()
}

//         // Assuming `children` is a Vec<Comment> and needs processing based on your application
//         // logic Placeholder for sorting logic, adapt as needed
//         // let mut sorted_children = comment_result.children;
//         // sorted_children
//         //   .sort_by(|a, b| a.points.cmp(&b.points).then_with(|| a.created.cmp(&b.created)));

//         // let response = CommentResponse {
//         //   text:     processed_text,
//         //   children: sorted_children,
//         //   // Populate other fields as necessary
//         // };
//         Ok::<Comment, MyError>(comment)
//       }
//       .scope_boxed()
//     })
//     .await?;

//   Ok(Json(comment))
// }

// /// Query for comment with `comment_id`
// /// update the `user_votes` table with the new vote
// /// Increment comment author's karma by 1
// /// Increment comment's points by 1
// pub async fn upvote_comment(
//   State(state): State<SharedState>,
//   Path(comment_id): Path<Uuid>,
//   Path(user_id): Path<Uuid>,
// ) -> Result<StatusCode, MyError> {
//   let conn = &mut *state.pool.get().await?;
//   let comment = conn
//     .transaction(|conn| {
//       async move {
//         let comment = comments_dsl.filter(comments::id.eq(comment_id)).first(conn).await?;
//         // let user_vote = models::user::vote(&comment.by, "comment", comment_id, None, true);
//         // diesel::insert_into(user_votes::table).values(&user_vote).execute(conn).await?;
//         Ok::<Comment, MyError>(comment)
//       }
//       .scope_boxed()
//     })
//     .await?;

//   Ok(StatusCode::OK)
// }

// // downvote_comment
// // unvote_comment
// // favorite_comment
// // unavorite_comment
// // get_edit_comment_page_data
// // edit_comment
// // get_delete_comment_page_data
// // delete_comment
// // get_replay_page_data
// // get_newest_comments_by_page
// // ...
