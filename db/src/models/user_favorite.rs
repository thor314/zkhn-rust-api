use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct UserFavorite {
  pub username:  String,
  pub item_type: String,
  pub item_id:   Uuid,
  pub date:      crate::utils::Timestamp,
}
