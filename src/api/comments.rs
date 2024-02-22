use axum::{extract::State, http::StatusCode, Json};
use diesel::prelude::*;
use diesel_async::{
  scoped_futures::ScopedFutureExt, AsyncConnection, AsyncPgConnection, RunQueryDsl,
};
use uuid::Uuid as Uid;

use crate::{
  error::MyError,
  models::{
    self,
    comment::{Comment, NewCommentPayload},
  },
  schema::comments::{self, dsl::comments as comments_dsl},
  SharedState,
};

// todo: auth
pub async fn add_new_comment(
  Json(payload): Json<NewCommentPayload>,
  State(state): State<SharedState>,
) -> Result<StatusCode, MyError> {
  let new_comment = Comment::from(payload);

  // Insert into database using Diesel
  let conn = &mut *state.pool.get().await?;

  // use a transaction, such that all operations are atomic; fail or succeed together
  conn
    .transaction(|conn| {
      async move {
        diesel::insert_into(comments::table).values(&new_comment).execute(conn).await?;
        models::user::increment_karma(conn, &new_comment.by).await?;
        models::item::increment_comments(conn, new_comment.parent_item_id).await?;
        if !new_comment.dead {
          // todo: search api: if user is not shadow-banned, tell the search api about the new
          // comment
        }

        Ok::<(), MyError>(())
      }
      .scope_boxed()
    })
    .await?;

  Ok(StatusCode::CREATED)
}
