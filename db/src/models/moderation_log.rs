use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a single moderation action taken by a moderator.
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct ModerationLog {
  /// The unique identifier for the log entry.
  pub id:                 Uuid,
  /// The username of the moderator who took the action.
  pub moderator_username: String,
  /// The type of action the moderator took. This will be one of several specified strings.
  pub action_type:        ModeratorAction,
  /// Username of the user the moderator action is related to.
  pub username:           Option<String>,
  /// ID of the item the moderator action was taken on.
  pub item_id:            Option<Uuid>,
  /// Title of the item the moderator action was taken on.
  pub item_title:         Option<String>,
  /// Author's username of the item the moderator action was taken on.
  pub item_by:            Option<String>,
  /// ID of the comment the moderator action was taken on.
  pub comment_id:         Option<Uuid>,
  /// Author's username of the comment the moderator action was taken on.
  pub comment_by:         Option<String>,
  /// UNIX timestamp that represents when the moderator action was taken.
  pub created:            NaiveDateTime,
}

// todo: extend
#[derive(Debug, Serialize, Deserialize)]
pub enum ModeratorAction {
  KillItem,
  UnkillItem,
  KillComment,
  UnkillComment,
  AddUserShadowBan,
  RemoveUserShadowBan,
  AddUserBan,
  RemoveUserBan,
}

impl ModerationLog {
  pub fn new(
    moderator_username: String,
    action_type: ModeratorAction,
    username: Option<String>,
    item_id: Option<Uuid>,
    item_title: Option<String>,
    item_by: Option<String>,
    comment_id: Option<Uuid>,
    comment_by: Option<String>,
  ) -> Self {
    ModerationLog {
      id: Uuid::new_v4(),
      moderator_username,
      action_type,
      username,
      item_id,
      item_title,
      item_by,
      comment_id,
      comment_by,
      created: crate::utils::now(),
    }
  }
}
