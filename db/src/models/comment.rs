// use axum::{extract::State, response::IntoResponse};
use super::*;

/// Comments on a post
#[derive(sqlx::FromRow, Debug, Serialize, Encode, Clone, Deserialize)]
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

impl Default for Comment {
  fn default() -> Self {
    Comment {
      id:                Uuid::new_v4(),
      username:          Username::default(),
      parent_item_id:    Uuid::new_v4(),
      parent_item_title: Title::default(),
      comment_text:      CommentText::default(),
      is_parent:         false,
      root_comment_id:   Uuid::new_v4(),
      parent_comment_id: None,
      children_count:    0,
      points:            1,
      created:           now(),
      dead:              false,
    }
  }

}

impl Comment {
  pub fn new(
    username: Username,
    parent_item_id: Uuid,
    parent_item_title: Title,
    is_parent: bool,
    root_comment_id: Option<Uuid>,
    parent_comment_id: Option<Uuid>,
    comment_text: CommentText,
    dead: bool,
  ) -> Self {
    // if root_comment_id is None, then this is the root comment
    let root_comment_id = root_comment_id.unwrap_or(Uuid::new_v4());
    // let text = crate::utils::sanitize_text(&text); // todo

    Comment {
      username,
      parent_item_id,
      parent_item_title,
      is_parent,
      root_comment_id,
      parent_comment_id,
      comment_text,
      dead,
      ..Default::default()
    }
  }

  pub fn increment_point(&mut self) { self.points += 1; }

  pub fn decrement_point(&mut self) {
    self.points = std::cmp::max(MIN_COMMENT_POINTS, self.points - 1);
  }

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
