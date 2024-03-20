use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Username;

// todo: this should have a uuid primary key
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct UserFavorite {
  pub username:  Username,
  /// comment or item
  pub item_type: String,
  pub item_id:   Uuid,
  pub date:      crate::utils::Timestamp,
}
