use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{now, Timestamp};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
/// Represents a vote cast by a user on an item or comment.
pub struct UserVote {
  /// The username of the user who cast the vote.
  pub username:       String,
  /// The type of content voted on.
  /// Item, Comment
  pub vote_type:      String,
  /// The ID of the item or comment voted on.
  pub content_id:     Uuid,
  /// The ID of the parent item for votes on comments.
  pub parent_item_id: Option<Uuid>,
  /// Indicates if the vote was an upvote.
  pub upvote:         bool,
  /// Indicates if the vote was a downvote (comments only).
  pub downvote:       bool,
  /// When the vote was cast.
  pub date:           Timestamp,
}

impl UserVote {
  pub fn new(
    username: String,
    vote_type: String,
    content_id: Uuid,
    parent_item_id: Option<Uuid>,
    upvote: bool,
    downvote: bool,
  ) -> Self {
    Self { username, vote_type, content_id, parent_item_id, upvote, downvote, date: now() }
  }
}
