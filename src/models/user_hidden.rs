use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

/// Represents an item that a user has chosen to hide.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserHidden {
    /// The username of the user who chose to hide this item.
    pub username: String,
    /// The ID of the hidden item.
    pub item_id: Uuid,
    /// A UNIX timestamp representing when the user set this item to be hidden.
    pub date: DateTime<Utc>,
    /// Date of the hidden item's creation.
    pub item_creation_date: DateTime<Utc>,
}
