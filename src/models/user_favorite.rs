use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

/// Represents a user's favorite item, including type and timestamp.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserFavorite {
  pub username:  String, // The username of the user who favorited this item.
  pub item_type: String, // Type of favorited item.
  pub item_id:   Uuid,   // The id of the favorited item.
  pub date:      DateTime<Utc>, // When the item was favorited, in UNIX timestamp.
}
