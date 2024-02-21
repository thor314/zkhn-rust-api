use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use diesel::{prelude::*, sql_types::*, QueryDsl, Queryable, Selectable, SelectableHelper};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid as Uid;

use crate::{error::MyError, schema::comments};

/// the minimum points a comment can have
const MIN_POINTS: i32 = -4;

/// Comments on a post
#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
// match to a schema for selectable
#[diesel(table_name = comments)]
// use postgres, improve compiler error messages.
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
  /// the unique identifier given to each comment in the form of a randomly generated string
  pub id:                Uid, // Assuming UUIDs for unique identifiers, common in SQL databases
  /// username of the user who created the comment
  pub by:                String,
  /// the id of the item the comment was placed on
  pub parent_item_id:    Uid,
  /// the title of the item the comment was placed on
  pub parent_item_title: String,
  /// body text for the comment
  pub text:              String,
  /// a boolean value that indicates whether or not the comment is a parent comment(not a child of
  /// any other comment)
  pub is_parent:         bool,
  /// a unique identifier for the root comment of a child comment, or else self
  pub root_comment_id:   Uid,
  /// the id of the parent comment. This will only be added if the comment is a direct reply to
  /// another comment
  pub parent_comment_id: Option<Uid>,
  pub children_count:    i32,
  /// sum total of upvotes and downvotes the comment has received. The minimum point value for a
  /// comment is -4
  pub points:            i32,
  pub created:           NaiveDateTime,
  /// Dead comments cannot be commented on, and are not displayed by default
  pub dead:              bool,
}

async fn child_comments(
  mut conn: AsyncPgConnection,
  id: Uid,
  show_dead_comments: bool,
) -> Result<Vec<Comment>, MyError> {
  // boxed does happy type-erasure magic for us
  let mut query =
    comments::dsl::comments.filter(comments::parent_comment_id.eq(Some(id))).into_boxed();
  if !show_dead_comments {
    query = query.filter(comments::dead.eq(false));
  }
  let result = query.select(Comment::as_select()).load(&mut conn).await.unwrap();
  Ok(result)
}

impl Comment {
  pub fn new(
    by: String,
    parent_item_id: Uid,
    parent_item_title: String,
    is_parent: bool,
    root_comment_id: Option<Uid>,
    parent_comment_id: Option<Uid>,
    text: String,
  ) -> Self {
    // if root_comment_id is None, then this is the root comment
    let root_comment_id = root_comment_id.unwrap_or(Uid::new_v4());
    Comment {
      id: Uid::new_v4(),
      by,
      parent_item_id,
      parent_item_title,
      is_parent,
      root_comment_id,
      parent_comment_id,
      text,
      children_count: 0,
      points: 1,
      created: crate::utils::now(),
      dead: false,
    }
  }

  pub fn edit(&mut self, text: String) { self.text = text; }

  pub fn increment_point(&mut self) { self.points += 1; }

  pub fn decrement_point(&mut self) { self.points = std::cmp::max(MIN_POINTS, self.points - 1); }

  // todo: set children to dead?
  pub fn kill(&mut self) { self.dead = true }

  pub fn unkill(&mut self) { self.dead = true }

  pub fn create_child_comment(&mut self, by: String, text: String) -> Comment {
    let comment = Comment::new(
      by,
      self.parent_item_id,
      self.parent_item_title.clone(),
      false,
      Some(self.root_comment_id),
      Some(self.id),
      text,
    );

    self.children_count += 1;
    comment
  }
}
