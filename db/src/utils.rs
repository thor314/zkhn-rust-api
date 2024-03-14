use anyhow::{anyhow, Context};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::Decode;
use tracing::trace;
use tracing_subscriber::{
  filter::{EnvFilter, LevelFilter},
  layer::SubscriberExt,
  util::SubscriberInitExt,
};
use validator::{Validate, ValidationError};

use crate::error::DbError;

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9A-Za-z_]+$").unwrap());

/// A timestamp wrapper that we can use in our sqlx models.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Timestamp(pub DateTime<Utc>);

pub fn now() -> Timestamp { Utc::now().into() }

impl From<DateTime<Utc>> for Timestamp {
  fn from(dt: DateTime<Utc>) -> Self { Timestamp(dt) }
}

// todo: test. Most of this should probably be done with a crate like ammonia, plus latex rendering?
/// Sanitize text:
/// - Trim whitespace
/// - Remove HTML tags
/// - Parse Markdown
/// - Parse Latex todo
/// - Linkify URLs
/// - Prevent XSS attacks with `ammonia`
pub fn sanitize_text(text: &str) -> String {
  let mut text = text.to_string();
  text = text.trim().to_string();
  // Remove HTML Tags
  let re_tags = Regex::new(r"<[^>]+>").unwrap();
  text = re_tags.replace_all(&text, "").to_string();
  // Replace Markdown-like Italic Syntax
  let re_italic = Regex::new(r"\*([^*]+)\*").unwrap();
  text = re_italic.replace_all(&text, "<i>$1</i>").to_string();
  // Linkify URLs (Simplified Example)
  // This is a placeholder. For actual URL detection and linkification,
  // you would need a more sophisticated approach or an external crate.
  let re_url = Regex::new(r"http://[^\s]+").unwrap(); // Simplified URL regex
  text = re_url.replace_all(&text, "<a href=\"$0\">$0</a>").to_string();
  // Prevent XSS Attacks
  text = ammonia::clean(&text);
  text
}

pub(crate) fn validate_username(username: &str) -> Result<(), ValidationError> {
  if username.len() < 3 {
    return Err(ValidationError::new("username_length must be greater than 3"));
  } else if username.len() > 16 {
    return Err(ValidationError::new("username_length must be less than 16"));
  } else if !USERNAME_REGEX.is_match(username) {
    return Err(ValidationError::new(
      "username must only contain alphanumeric characters and underscores",
    ));
  }

  Ok(())
}
