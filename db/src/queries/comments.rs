use uuid::Uuid;

use crate::{
  error::DbError,
  models::{
    comment::Comment,
    user_vote::{UserVote, VoteState},
  },
  utils::now,
  DbPool, DbResult,
};

/// Via the atomic sqlx transaction api:
/// - insert new comment into db
/// - increment user karma
/// - increment item comment count
pub async fn insert_comment(
  pool: &sqlx::Pool<sqlx::Postgres>,
  new_comment: &Comment,
) -> DbResult<()> {
  let mut tx = pool.begin().await?;

  let Comment {
    id,
    comment_text,
    is_parent,
    root_comment_id,
    parent_comment_id,
    created,
    dead,
    username,
    parent_item_id,
    parent_item_title,
    children_count,
    points,
  } = new_comment;

  sqlx::query!(
    "INSERT INTO comments 
    (id, username, parent_item_id, comment_text, is_parent, root_comment_id, parent_comment_id, \
     created, dead) 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    id,
    username,
    parent_item_id,
    comment_text,
    is_parent,
    root_comment_id,
    // parent_comment_id, // can't provide an Option<Uuid>, potential source of bugs
    parent_comment_id.map_or(Uuid::nil(), Uuid::from),
    created.0,
    dead
  )
  .execute(&mut *tx)
  .await?;

  // Increment user karma
  sqlx::query!("UPDATE users SET karma = karma + 1 WHERE username = $1", username)
    .execute(&mut *tx)
    .await?;

  // Increment item comment count
  sqlx::query!("UPDATE items SET comment_count = comment_count + 1 WHERE id = $1", parent_item_id)
    .execute(&mut *tx)
    .await?;

  // todo: tell the search api about the new comment

  tx.commit().await?;

  Ok(())
}

pub async fn get_comment_by_id(pool: &DbPool, comment_id: Uuid) -> DbResult<Option<Comment>> {
  sqlx::query_as!(Comment, "SELECT * FROM comments WHERE id = $1", comment_id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn get_comment_children_of(pool: &DbPool, comment_id: Uuid) -> DbResult<Vec<Comment>> {
  sqlx::query_as!(Comment, "SELECT * FROM comments WHERE parent_comment_id = $1", comment_id)
    .fetch_all(pool)
    .await
    .map_err(DbError::from)
}

/// recursively remove a comment from the database
pub async fn delete_comment(pool: &DbPool, comment_id: Uuid) -> DbResult<()> {
  let comments: Vec<Comment> = get_comment_children_of(pool, comment_id).await?;
  sqlx::query!("DELETE FROM comments WHERE id = $1", comment_id)
    .execute(pool)
    .await
    .map_err(DbError::from)?;

  for comment in comments {
    // recursive async requires Box::pin
    Box::pin(delete_comment(pool, comment.id)).await?;
  }

  // TODO(TK 2024-03-13): update item comment count
  // update user karma
  // update search api

  Ok(())
}
