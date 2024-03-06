#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

pub mod error;
pub mod models;
#[cfg(test)] mod tests;
mod utils;

use error::DbError;
use models::{item::Item, user::User, user_vote::UserVote};
use uuid::Uuid;

use crate::{models::comment::Comment, utils::now};

pub type DbPool = sqlx::postgres::PgPool;
pub type DbResult<T> = Result<T, DbError>;

pub async fn migrate(pool: &DbPool) -> Result<(), DbError> {
  sqlx::migrate!("../db/migrations").run(pool).await?;
  Ok(())
}

// todo: move to user-queries
pub async fn get_user_by_id(pool: &DbPool, id: Uuid) -> DbResult<Option<User>> {
  sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id) // syntax error at end of input
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn get_user_by_username(pool: &DbPool, username: &str) -> DbResult<Option<User>> {
  sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn get_item_by_id(pool: &DbPool, item_id: Uuid) -> DbResult<Option<Item>> {
  sqlx::query_as!(Item, "SELECT * FROM items WHERE id = $1", item_id)
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
    comment_text,
    is_parent,
    root_comment_id,
    parent_comment_id,
    created,
    dead,
    by,
    parent_item_id,
    parent_item_title,
    children_count,
    points,
  } = new_comment;

  sqlx::query!(
    "INSERT INTO comments 
    (id, by, parent_item_id, comment_text, is_parent, root_comment_id, parent_comment_id, created, \
     dead) 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    id,
    by,
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
  sqlx::query!("UPDATE users SET karma = karma + 1 WHERE username = $1", by)
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

pub async fn get_user_vote_by_content_id(
  pool: &DbPool,
  username: &str,
  content_id: Uuid,
) -> DbResult<Option<UserVote>> {
  sqlx::query_as!(
    UserVote,
    "SELECT * FROM user_votes WHERE content_id = $1 and username = $2",
    content_id,
    username
  )
  .fetch_optional(pool)
  .await
  .map_err(DbError::from)
}

/// submit an upvote on a comment in the db. Assume the user has not already upvoted the comment
/// (verified in API)
pub async fn upvote_comment(
  pool: &mut sqlx::Pool<sqlx::Postgres>,
  comment_id: Uuid,
  user_name: &str,
  parent_item_id: Uuid,
) -> DbResult<()> {
  let mut tx = pool.begin().await?;
  // Insert user vote
  sqlx::query!(
    "INSERT INTO user_votes (username, vote_type, content_id, parent_item_id, upvote, downvote, \
     date)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    user_name,
    "comment",
    comment_id,
    parent_item_id,
    true,
    false,
    now().0,
  )
  .execute(&mut *tx)
  .await?;

  // Update comment points (adjust query if points are stored differently)
  sqlx::query!("UPDATE comments SET points = points + 1 WHERE id = $1", comment_id)
    .execute(&mut *tx)
    .await?;

  // Update user karma (implement logic here, assuming a `users` table with `karma` field)
  sqlx::query!("UPDATE users SET karma = karma + 1 WHERE username = $1", user_name,)
    .execute(&mut *tx)
    .await?;

  tx.commit().await?;
  Ok(())
}
