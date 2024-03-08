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
  pub vote_state:     VoteState,
  /// When the vote was cast.
  pub created:        Timestamp,
}

impl UserVote {
  pub fn new(
    username: String,
    vote_type: String,
    content_id: Uuid,
    parent_item_id: Option<Uuid>,
    vote_state: VoteState,
  ) -> Self {
    Self { username, vote_type, content_id, parent_item_id, vote_state, created: now() }
  }
}

#[derive(sqlx::Type, PartialEq, Serialize, Deserialize, Debug, Clone)]
#[sqlx(type_name = "vote_state")] // only for PostgreSQL to match a type definition
#[sqlx(rename_all = "lowercase")]
pub enum VoteState {
  Upvote,
  Downvote,
  None,
}

impl From<i8> for VoteState {
  fn from(v: i8) -> Self {
    match v {
      1 => Self::Upvote,
      0 => Self::None,
      -1 => Self::Downvote,
      _ => Self::None,
    }
  }
}
