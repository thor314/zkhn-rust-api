use axum::{extract::State, http::StatusCode, Json};
use diesel_async::RunQueryDsl;

use crate::{
  error::MyError,
  models::comment::{Comment, NewCommentPayload},
  schema::comments,
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

  Ok(StatusCode::CREATED)
}
