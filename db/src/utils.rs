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

use crate::{error::DbError, Timestamp};

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9A-Za-z_]+$").unwrap());

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
