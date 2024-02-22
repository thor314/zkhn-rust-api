use axum::{extract::State, http::StatusCode, Json};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid as Uid;

use crate::{
  error::MyError,
  models::comment::{Comment, NewCommentPayload},
  schema::{comments, comments::dsl::comments as comments_dsl},
  SharedState,
};

pub async fn add_new_comment(
  Json(payload): Json<NewCommentPayload>,
  State(state): State<SharedState>,
) -> Result<StatusCode, MyError> {
  let new_comment = Comment::from(payload);

  // Insert into database using Diesel
  let mut conn = state.pool.get().await?;
  diesel::insert_into(comments::table).values(new_comment).execute(&mut conn).await?;

  // crate::models::user::increment_karma(conn, &new_comment.by).await?;
  // crate::models::item::increment_comments(conn, new_comment.parent_item_id).await?;


  Ok(StatusCode::CREATED)
}
