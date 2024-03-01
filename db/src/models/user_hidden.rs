use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct UserHidden {
  /// The username of the user who chose to hide this item.
  pub username:           String,
  /// The ID of the hidden item.
  pub item_id:            Uuid,
  /// A UNIX timestamp representing when the user set this item to be hidden.
  pub date:               NaiveDateTime,
  /// Date of the hidden item's creation.
  pub item_creation_date: NaiveDateTime,
}
