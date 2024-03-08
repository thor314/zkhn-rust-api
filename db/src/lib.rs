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

use crate::{
  models::{comment::Comment, user_favorite::UserFavorite},
  utils::now,
};

pub type DbPool = sqlx::postgres::PgPool;
pub type DbResult<T> = Result<T, DbError>;

pub async fn migrate(pool: &DbPool) -> Result<(), DbError> {
  sqlx::migrate!("../db/migrations").run(pool).await?;
  Ok(())
}

// todo: move comments into comments
pub use crate::{comments::*, items::*, user_votes::*, users::*};
mod users {
  use uuid::Uuid;

  use crate::{
    error::DbError,
    models::{comment::Comment, user::User},
    DbPool, DbResult,
  };
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
}

mod items {
  use uuid::Uuid;

  use crate::{
    error::DbError,
    models::{comment::Comment, item::Item},
    DbPool, DbResult,
  };

  pub async fn get_item_by_id(pool: &DbPool, item_id: Uuid) -> DbResult<Option<Item>> {
    sqlx::query_as!(Item, "SELECT * FROM items WHERE id = $1", item_id)
      .fetch_optional(pool)
      .await
      .map_err(DbError::from)
  }
}

mod comments {
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
    sqlx::query!(
      "UPDATE items SET comment_count = comment_count + 1 WHERE id = $1",
      parent_item_id
    )
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

  /// submit an upvote on a comment in the db. Assume the user has not already upvoted the comment
  /// (verified in API)
  pub async fn submit_comment_vote(
    pool: &mut sqlx::Pool<sqlx::Postgres>,
    comment_id: Uuid,
    user_name: &str,
    parent_item_id: Uuid,
    vote_state: VoteState,
  ) -> DbResult<()> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
      "INSERT INTO user_votes (username, vote_type, content_id, parent_item_id, vote_state, \
       created)
         VALUES ($1, $2, $3, $4, $5, $6)",
      user_name,
      "comment",
      comment_id,
      parent_item_id,
      VoteState::Upvote as _,
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
}

pub async fn get_user_favorite_by_username_and_item_id(
  pool: &DbPool,
  username: &str,
  item_id: Uuid,
) -> DbResult<Option<UserFavorite>> {
  sqlx::query_as!(
    UserFavorite,
    "SELECT username, item_type, item_id, date
       FROM user_favorites WHERE item_id = $1 and username = $2",
    item_id,
    username
  )
  .fetch_optional(pool)
  .await
  .map_err(DbError::from)
}

mod user_votes {
  use uuid::Uuid;

  use crate::{error::DbError, models::user_vote::UserVote, DbPool, DbResult};

  pub async fn get_user_vote_by_content_id(
    pool: &DbPool,
    username: &str,
    content_id: Uuid,
  ) -> DbResult<Option<UserVote>> {
    sqlx::query_as!(
      UserVote,
      "SELECT username, vote_type, content_id, parent_item_id, vote_state as \"vote_state: _\", \
       created FROM user_votes WHERE content_id = $1 and username = $2",
      content_id,
      username
    )
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
  }
}

/// Insert a new user favorite for comment
pub async fn insert_or_delete_user_favorite_for_comment(
  pool: &sqlx::Pool<sqlx::Postgres>,
  user_name: &str,
  maybe_favorite: Option<UserFavorite>,
  comment_id: Uuid,
) -> DbResult<()> {
  match maybe_favorite {
    Some(favorite) => {
      sqlx::query!(
        "DELETE FROM user_favorites 
      WHERE item_id = $1",
        favorite.item_id,
      )
      .execute(pool)
      .await?;
      Ok(())
    }
    None => {
      sqlx::query!(
        "INSERT INTO user_favorites (username, item_type, item_id, date)
         VALUES ($1, $2, $3, $4)",
        user_name,
        "comment",
        comment_id,
        now().0,
      )
      .execute(pool)
      .await?;
      Ok(())
    }
  }
}

// pub async fn get_user_vote_by_content_id(
//   pool: &DbPool,
//   username: &str,
//   content_id: Uuid,
// ) -> DbResult<Option<UserVote>> {
//   sqlx::query_as!(
//     UserVote,
//     "SELECT * FROM user_votes WHERE content_id =  and username = ",
//     content_id,
//     username
//   )
//   .fetch_optional(pool)
//   .await
//   .map_err(DbError::from)
// }
