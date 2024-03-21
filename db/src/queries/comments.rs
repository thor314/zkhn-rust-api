use std::collections::HashSet;

use futures::future::join_all;
use rayon::prelude::*;
use sqlx::{Pool, Postgres, QueryBuilder, Transaction};
use uuid::Uuid;

use crate::{
  error::DbError,
  models::{
    comment::{self, Comment},
    item::Item,
    user_vote::{UserVote, VoteState},
  },
  utils::now,
  CommentText, DbPool, DbResult, Title, Username,
};

pub async fn get_comment(pool: &DbPool, comment_id: Uuid) -> DbResult<Option<Comment>> {
  sqlx::query_as!(
    Comment,
    "SELECT 
    id,
    username as \"username: Username\",
    parent_item_id,
    parent_item_title as \"parent_item_title: Title\",
    comment_text as \"comment_text: CommentText\",
    is_parent,
    root_comment_id,
    parent_comment_id,
    children_count,
    points,
    created,
    dead
   FROM comments WHERE id = $1",
    comment_id
  )
  .fetch_optional(pool)
  .await
  .map_err(DbError::from)
}

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
    username,
    parent_item_id,
    parent_item_title,
    comment_text,
    is_parent,
    root_comment_id,
    parent_comment_id,
    children_count,
    points,
    created,
    dead,
  } = new_comment.clone();

  sqlx::query!(
    "INSERT INTO comments 
    ( id, 
      username, 
      parent_item_id, 
      parent_item_title, 
      comment_text, 
      is_parent, 
      root_comment_id, 
      parent_comment_id, 
      children_count,
      points,
      created,
      dead ) 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
    id,
    username.0,
    parent_item_id,
    parent_item_title.0,
    comment_text.0,
    is_parent,
    root_comment_id,
    parent_comment_id,
    children_count,
    points,
    created.0,
    dead
  )
  .execute(&mut *tx)
  .await?;

  // Increment user karma
  sqlx::query!("UPDATE users SET karma = karma + 1 WHERE username = $1", username.0)
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

pub async fn get_comment_children_layer(pool: &DbPool, comment_id: Uuid) -> DbResult<Vec<Comment>> {
  sqlx::query_as!(
    Comment,
    "SELECT 
    id,
    username as \"username: Username\",
    parent_item_id,
    parent_item_title as \"parent_item_title: Title\",
    comment_text as \"comment_text: CommentText\",
    is_parent,
    root_comment_id,
    parent_comment_id,
    children_count,
    points,
    created,
    dead
  FROM comments WHERE parent_comment_id = $1",
    comment_id
  )
  .fetch_all(pool)
  .await
  .map_err(DbError::from)
}

async fn get_comment_children_recursive(pool: &DbPool, comment_id: Uuid) -> DbResult<Vec<Uuid>> {
  let children_ids = get_comment_children_layer(pool, comment_id).await?;
  let children_futures =
    children_ids.into_iter().map(|row| get_comment_children_recursive(pool, row.id));

  let results: DbResult<Vec<_>> = join_all(children_futures).await.into_iter().collect();
  let results = results?.into_iter().flatten().collect();
  Ok(results)
}

/// recursively get all comment_id's to remove, then remove them in a single transaction
pub async fn delete_comment(pool: &DbPool, comment_id: Uuid, item_id: Uuid) -> DbResult<()> {
  let mut comments_to_delete = get_comment_children_recursive(pool, comment_id).await?;
  comments_to_delete.push(comment_id);

  // todo: optimize. This should probably happen via postgres triggers instead.
  // delete all comments in a transaction
  let mut tx = pool.begin().await?;
  for comment_id in &comments_to_delete {
    sqlx::query!("DELETE FROM comments WHERE id = $1", comment_id).execute(&mut *tx).await?;
  }
  tx.commit().await?;

  super::items::decrement_item_comment_count_by(pool, item_id, comments_to_delete.len() as i32)
    .await?;
  // TODO(TK 2024-03-13): update item comment count
  // update user karma
  // update search api

  Ok(())
}

// pub async fn child_comments(
//   mut conn: &DbPool,
//   id: Uuid,
//   show_dead_comments: bool,
// ) -> DbResult<Vec<Comment>> {
//   let comments: Vec<Comment> = sqlx::query_as!(
//     Comment,
//     "SELECT
//       id,
//       username as \"username: Username\",
//       parent_item_id,
//       parent_item_title as \"parent_item_title: Title\",
//       comment_text as \"comment_text: CommentText\",
//       is_parent,
//       root_comment_id,
//       parent_comment_id,
//       children_count,
//       points,
//       created,
//       dead
//     FROM comments WHERE parent_comment_id = $1",
//     id
//   )
//   .fetch_all(&mut conn)
//   .await?;

//   Ok(comments)
// }
