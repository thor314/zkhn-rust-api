use anyhow::{anyhow, Context};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use tracing::trace;
use tracing_subscriber::{
  filter::{EnvFilter, LevelFilter},
  layer::SubscriberExt,
  util::SubscriberInitExt,
};

pub fn now() -> NaiveDateTime {
  NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap()
}
