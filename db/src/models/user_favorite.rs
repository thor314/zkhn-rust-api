use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct UserFavorite {
  pub username:  String,
  pub item_type: String,
  pub item_id:   Uuid,
  pub date:      NaiveDateTime,
}
