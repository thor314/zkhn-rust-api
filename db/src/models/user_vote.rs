use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
/// Represents a vote cast by a user on an item or comment.
pub struct UserVote {
  /// The username of the user who cast the vote.
  pub username:       String,
  /// The type of content voted on.
  pub vote_type:      VoteType,
  /// The ID of the item or comment voted on.
  pub content_id:     Uuid,
  /// The ID of the parent item for votes on comments.
  pub parent_item_id: Option<Uuid>,
  /// Indicates if the vote was an upvote.
  pub upvote:         bool,
  /// Indicates if the vote was a downvote (comments only).
  pub downvote:       bool,
  /// When the vote was cast.
  pub date:           NaiveDateTime,
}

/// Defines the type of content a vote is associated with.
#[derive(Debug, Serialize, Deserialize)]
pub enum VoteType {
  Item,
  Comment,
}
