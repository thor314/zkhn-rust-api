// use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, NaiveDate, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode};
use uuid::Uuid;

use crate::{
  error::DbError,
  utils::{now, Timestamp},
  CommentText, DbResult, Title, Username,
};

/// the minimum points a comment can have
const MIN_POINTS: i32 = -4;

/// Comments on a post
#[derive(sqlx::FromRow, Debug, Serialize, Encode, Clone)]
pub struct Comment {
  /// the unique identifier given to each comment in the form of a randomly generated string
  pub id:                Uuid, // Assuming UUIDs for unique identifiers, common in SQL databases
  /// username of the user who created the comment
  pub username:          Username,
  /// the id of the item the comment was placed on
  pub parent_item_id:    Uuid,
  /// the title of the item the comment was placed on
  pub parent_item_title: Title,
  /// body text for the comment
  pub comment_text:      CommentText, // validate
  /// a boolean value that indicates whether or not the comment is a parent comment(not a child of
  /// any other comment)
  pub is_parent:         bool,
  /// a unique identifier for the root comment of a child comment, or else self
  pub root_comment_id:   Uuid,
  /// the id of the parent comment. This will only be added if the comment is a direct reply to
  /// another comment
  pub parent_comment_id: Option<Uuid>,
  pub children_count:    i32,
  /// sum total of upvotes and downvotes the comment has received. The minimum point value for a
  /// comment is -4
  pub points:            i32,
  pub created:           Timestamp,
  /// Dead comments cannot be commented on, and are not displayed by default.
  /// Comments submitted by shadow-banned users are dead.
  pub dead:              bool,
}

impl Comment {
  pub fn new(
    username: Username,
    parent_item_id: Uuid,
    parent_item_title: Title,
    is_parent: bool,
    root_comment_id: Option<Uuid>,
    parent_comment_id: Option<Uuid>,
    text: CommentText,
    dead: bool,
  ) -> Self {
    // if root_comment_id is None, then this is the root comment
    let root_comment_id = root_comment_id.unwrap_or(Uuid::new_v4());
    // let text = crate::utils::sanitize_text(&text); // todo

    Comment {
      id: Uuid::new_v4(),
      username,
      parent_item_id,
      parent_item_title,
      is_parent,
      root_comment_id,
      parent_comment_id,
      comment_text: text,
      children_count: 0,
      points: 1,
      created: now(),
      dead,
    }
  }

  pub fn edit(&mut self, text: CommentText) { self.comment_text = text; }

  pub fn increment_point(&mut self) { self.points += 1; }

  pub fn decrement_point(&mut self) { self.points = std::cmp::max(MIN_POINTS, self.points - 1); }

  // todo: set children to dead?
  pub fn kill(&mut self) { self.dead = true }

  pub fn unkill(&mut self) { self.dead = true }

  pub fn create_child_comment(&mut self, by: Username, text: CommentText, dead: bool) -> Comment {
    let comment = Comment::new(
      by,
      self.parent_item_id,
      self.parent_item_title.clone(),
      false,
      Some(self.root_comment_id),
      Some(self.id),
      text,
      dead,
    );

    self.children_count += 1;
    comment
  }
}

pub async fn child_comments(
  mut conn: sqlx::PgConnection,
  id: Uuid,
  show_dead_comments: bool,
) -> Result<Vec<Comment>, DbError> {
  let comments: Vec<Comment> = sqlx::query_as!(
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
    id
  )
  .fetch_all(&mut conn)
  .await?;

  Ok(comments)
}

// corresponding to `add_new_comment` in API
#[derive(Debug, Deserialize, Validate)]
pub struct NewCommentPayload {
  #[garde(dive)]
  username:          Username,
  #[garde(skip)]
  parent_item_id:    Uuid,
  #[garde(dive)]
  parent_item_title: Title,
  #[garde(skip)]
  is_parent:         bool,
  #[garde(skip)]
  root_comment_id:   Option<Uuid>,
  #[garde(skip)]
  parent_comment_id: Option<Uuid>,
  #[garde(dive)]
  text:              CommentText,
  #[garde(skip)]
  dead:              bool,
}

// todo: tryfrom
impl TryFrom<NewCommentPayload> for Comment {
  type Error = DbError;

  fn try_from(payload: NewCommentPayload) -> DbResult<Self> {
    payload.validate(&())?;
    let comment = Comment::new(
      payload.username,
      payload.parent_item_id,
      payload.parent_item_title,
      payload.is_parent,
      payload.root_comment_id,
      payload.parent_comment_id,
      payload.text,
      payload.dead,
    );

    Ok(comment)
  }
}
