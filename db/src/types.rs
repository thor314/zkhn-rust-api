//! Newtype wrappers for input validation and type-safety
use std::fmt;

use chrono::{DateTime, TimeDelta, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, Decode, Encode};
use tracing::warn;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{utils::now, DbResult};

/// A timestamp wrapper that we can use in our sqlx models.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Type, PartialEq, PartialOrd)]
#[sqlx(transparent)]
pub struct Timestamp(pub DateTime<Utc>);
impl Default for Timestamp {
  fn default() -> Self { Self::now() }
}
impl Timestamp {
  pub fn now() -> Self { Timestamp(Utc::now()) }

  /// generate an expiration date
  pub fn default_token_expiration() -> Self { now() + TimeDelta::try_days(1).unwrap() }
}
impl std::ops::Add<TimeDelta> for Timestamp {
  type Output = Self;

  fn add(self, other: TimeDelta) -> Self { Timestamp(self.0 + other) }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Type)]
#[garde(transparent)]
#[repr(transparent)]
pub struct About(#[garde(ascii, length(min = 0, max = 400))] pub String);
impl Default for About {
  fn default() -> Self { "about ipsum dolor".into() }
}
impl From<&str> for About {
  fn from(s: &str) -> Self { About(s.to_string()) }
}

// NOTE: deriving ToSchema doesn't appear to make the docs clearer
// #[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq, Type, ToSchema)]
// #[schema(default = "Username(\"alice\")")]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq, Type)]
#[garde(transparent)]
#[repr(transparent)]
pub struct Username(#[garde(ascii, length(min = 3, max = 25))] pub String);
impl fmt::Display for Username {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}
impl Default for Username {
  fn default() -> Self { "alice".into() }
}
impl From<&str> for Username {
  fn from(s: &str) -> Self { Username(s.to_string()) }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Type)]
#[garde(transparent)]
#[repr(transparent)]
pub struct Email(#[garde(email)] pub String);
impl Default for Email {
  fn default() -> Self { "email@email.com".into() }
}
impl std::fmt::Display for Email {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}
impl From<&str> for Email {
  fn from(s: &str) -> Self { Email(s.to_string()) }
}

/// A raw, unhashed password
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[garde(transparent)]
pub struct Password(#[garde(ascii, length(min = 8, max = 25))] pub String);
impl Default for Password {
  fn default() -> Self { "password".into() }
}
impl From<&str> for Password {
  fn from(s: &str) -> Self { Password(s.to_string()) }
}

/// A hashed password
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[repr(transparent)]
pub struct PasswordHash(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[repr(transparent)]
pub struct AuthToken(pub String);
impl Default for AuthToken {
  fn default() -> Self { AuthToken("default_auth_token".into()) }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Validate, PartialEq)]
#[repr(transparent)]
pub struct ResetPasswordToken(#[garde(ascii, length(min = 40, max = 40))] pub String);
impl Default for ResetPasswordToken {
  fn default() -> Self { ResetPasswordToken("1234567890123456789012345678901234567890".into()) }
}
impl From<&str> for ResetPasswordToken {
  fn from(s: &str) -> Self { ResetPasswordToken(s.to_string()) }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Validate, PartialEq)]
#[garde(transparent)]
#[repr(transparent)]
pub struct Title(#[garde(ascii, length(min = 8, max = 100))] pub String);
impl Default for Title {
  fn default() -> Self { "item title".into() }
}
impl From<&str> for Title {
  fn from(s: &str) -> Self { Title(s.to_string()) }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Validate)]
#[garde(transparent)]
#[repr(transparent)]
pub struct CommentText(#[garde(ascii, length(min = 8, max = 2000))] pub String);
impl Default for CommentText {
  fn default() -> Self { "comment ipsum dolor".into() }
}
impl From<&str> for CommentText {
  fn from(s: &str) -> Self { CommentText(s.to_string()) }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Validate)]
#[garde(transparent)]
/// A page of comments or items
pub struct Page {
  #[garde(range(min = 1, max = 1000))]
  pub page: i32,
}
impl Default for Page {
  fn default() -> Self { Self { page: 1 } }
}
impl From<i32> for Page {
  fn from(n: i32) -> Self { Self { page: n } }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Type)]
#[garde(transparent)]
pub struct Url(#[garde(url)] pub String);
impl Default for Url {
  fn default() -> Self { "http://example.com".into() }
}
impl From<&str> for Url {
  fn from(s: &str) -> Self { Self(s.into()) }
}
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Domain(pub String);
impl Default for Domain {
  fn default() -> Self { Domain("example.com".into()) }
}
impl From<Url> for Domain {
  fn from(d: Url) -> Self {
    let url = url::Url::parse(&d.0).unwrap();
    let domain = url.domain().unwrap().to_string();
    Self(domain)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Type)]
#[garde(transparent)]
pub struct Text(#[garde(ascii, length(min = 10, max = 2000))] pub String);
impl Default for Text {
  fn default() -> Self { "Some text for your reading pleasure".into() }
}
impl From<&str> for Text {
  fn from(s: &str) -> Self { Self(s.into()) }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(default = TextOrUrl::default, example=TextOrUrl::default)]
pub enum TextOrUrl {
  Text(#[garde(dive)] Text),
  Url(#[garde(dive)] Url),
}
impl TextOrUrl {
  pub fn url_domain_text(self) -> (Option<Url>, Option<Domain>, Option<Text>) {
    match self {
      TextOrUrl::Text(text) => (None, None, Some(text.clone())),
      TextOrUrl::Url(url) => (Some(url.clone()), Some(Domain::from(url)), None),
    }
  }
}
impl Default for TextOrUrl {
  fn default() -> Self { Self::Url(Url::default()) }
}
