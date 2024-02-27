use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use diesel::{prelude::*, sql_types::*, QueryDsl, Queryable, Selectable, SelectableHelper};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid as Uid;

use crate::schema::user_votes;

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
// match to a schema for selectable
#[diesel(table_name = user_votes)]
// use postgres, improve compiler error messages.
#[diesel(check_for_backend(diesel::pg::Pg))]
/// Represents a vote cast by a user on an item or comment.
pub struct UserVote {
  /// The username of the user who cast the vote.
  pub username:       String,
  /// The type of content voted on.
  pub vote_type:      VoteType,
  /// The ID of the item or comment voted on.
  pub content_id:     Uid,
  /// The ID of the parent item for votes on comments.
  pub parent_item_id: Option<Uid>,
  /// Indicates if the vote was an upvote.
  pub upvote:         bool,
  /// Indicates if the vote was a downvote (comments only).
  pub downvote:       bool,
  /// When the vote was cast.
  pub date:           NaiveDateTime,
}

/// Defines the type of content a vote is associated with.
#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::UserVoteType"]
pub enum VoteType {
  Item,
  Comment,
}
