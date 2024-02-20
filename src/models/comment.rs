use serde::{Deserialize, Serialize};
use sqlx::{
  query_as,
  types::{
    chrono::{DateTime, Utc},
    Uuid,
  },
};

/// the minimum points a comment can have
const MIN_POINTS: i32 = -4;

// NOTE(TK 2024-02-19): don't use `Ulid`s, they confuse sqlx :(
/// Comments on a post
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Comment {
  pub id:                Uuid, // Assuming UUIDs for unique identifiers, common in SQL databases
  pub by:                String,
  pub parent_item_id:    String,
  pub parent_item_title: String,
  pub is_parent:         bool,
  pub parent_comment_id: Option<Uuid>,
  pub text:              Option<String>, // todo: optional?
  pub points:            i32,
  pub created:           DateTime<Utc>,
  /// Dead comments cannot be commented on, and are not displayed by default
  pub dead:              bool,
}

async fn fetch_child_comments(
  pool: &sqlx::PgPool,
  id: &Uuid,
  show_dead_comments: bool,
) -> Result<Vec<Comment>, sqlx::Error> {
  let s = if show_dead_comments { "AND dead = true" } else { "" };
  let record =
    sqlx::query_as::<_, Comment>(r#"SELECT * FROM comments WHERE parent_comment_id = $1 $2"#)
      .bind(id)
      .bind(s)
      .fetch_all(pool)
      .await?;

  Ok(record)
}

impl Comment {
  fn new(
    by: String,
    parent_item_id: String,
    parent_item_title: String,
    is_parent: bool,
    parent_comment_id: Option<Uuid>,
    text: Option<String>,
  ) -> Self {
    Comment {
      id: Uuid::new_v4(),
      by,
      parent_item_id,
      parent_item_title,
      is_parent,
      parent_comment_id,
      text,
      points: 1,
      created: Utc::now(),
      dead: false,
    }
  }

  fn edit(&mut self, text: String) { self.text = Some(text); }

  fn increment_point(&mut self) { self.points += 1; }

  fn decrement_point(&mut self) { self.points = std::cmp::max(MIN_POINTS, self.points - 1); }

  fn kill(&mut self) { self.dead = true }

  fn unkill(&mut self) { self.dead = true }
}
