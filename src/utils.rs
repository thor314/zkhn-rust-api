use anyhow::{anyhow, Context};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use regex::Regex;
use tracing::trace;
use tracing_subscriber::{
  filter::{EnvFilter, LevelFilter},
  layer::SubscriberExt,
  util::SubscriberInitExt,
};

use crate::error::MyError;
/// Set up crate logging and environment variables.
pub(crate) fn setup(secret_store: &shuttle_secrets::SecretStore) -> Result<(), MyError> {
  let filter =
    EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy();
  tracing_subscriber::fmt().with_env_filter(filter).init();
  secret_store.get("DOTENV_OK").context("failed to get secrets")?;
  Ok(())
}

pub fn now() -> NaiveDateTime {
  NaiveDateTime::from_timestamp_opt(Utc::now().timestamp(), 0).unwrap()
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
