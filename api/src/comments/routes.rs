use axum::{
  extract::{Path, State},
  http::StatusCode,
  Json, Router,
};

use super::extractors::CommentExtractor;
// use sqlx::types::Uuid;
use crate::{
  auth::{assert_authenticated, AuthSession},
  error::{ApiError, RouteError},
  ApiResult, DbPool,
};

// models::{self},
//   comment::{Comment, NewCommentPayload},
// schema::{self, comments, comments::dsl::comments as comments_dsl},

pub async fn add_new_comment(
  State(pool): State<DbPool>,
  Json(payload): Json<CommentExtractor>,
  auth_session: AuthSession,
) -> ApiResult<StatusCode> {
  assert_authenticated(&auth_session)?;
  todo!()
}
//   let new_comment = Comment::from(payload);
//   let conn = &mut *state.pool.get().await?;
//   // transaction: all operations are atomic; fail or succeed together
//   conn
//     .transaction(|conn| {
//       async move {
//         diesel::insert_into(comments::table).values(&new_comment).execute(conn).await?;
//         models::user::increment_karma(conn, &new_comment.by).await?;
//         models::item::increment_comments(conn, new_comment.parent_item_id).await?;
//         if !new_comment.dead {
//           // todo: search api: if user is not shadow-banned, tell the search api about the new
//           // tell search api about
//           // - comment
//           // - item.id
//           // - item comment count increment
//         }

//         Ok::<(), MyError>(())
//       }
//       .scope_boxed()
//     })
//     .await?;

//   Ok(StatusCode::CREATED)
// }

// pub async fn get_comment_by_id(
//   State(state): State<SharedState>,
//   Path(comment_id): Path<Uuid>,
// ) -> Result<Json<Comment>, MyError> {
//   let conn = &mut *state.pool.get().await?;
//   let comment = conn
//     .transaction(|conn| {
//       async move {
//         // Step 1: Query for the comment
//         let comment: Comment =
// comments_dsl.filter(comments::id.eq(comment_id)).first(conn).await?;

//         // todo: not sure what this is for, leave commented
//         // let processed_text = comment_result.text.replace(/<[^>]+>/g, "");

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
