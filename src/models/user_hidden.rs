use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use diesel::{prelude::*, sql_types::*, QueryDsl, Queryable, Selectable, SelectableHelper};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid as Uid;

use crate::schema::user_hiddens;

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
// match to a schema for selectable
#[diesel(table_name = user_hiddens)]
// use postgres, improve compiler error messages.
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserHidden {
  /// The username of the user who chose to hide this item.
  pub username:           String,
  /// The ID of the hidden item.
  pub item_id:            Uid,
  /// A UNIX timestamp representing when the user set this item to be hidden.
  pub date:               NaiveDateTime,
  /// Date of the hidden item's creation.
  pub item_creation_date: NaiveDateTime,
}
