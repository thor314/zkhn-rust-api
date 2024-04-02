//! Newtype wrappers for input validation and type-safety
use std::fmt;

use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, Decode, Encode};
use tracing::warn;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{password::hash_password_argon, DbResult};

/// A timestamp wrapper that we can use in our sqlx models.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Timestamp(pub DateTime<Utc>);

impl Default for Timestamp {
  fn default() -> Self { Timestamp(Utc::now()) }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Type)]
#[garde(transparent)]
#[repr(transparent)]
pub struct About(#[garde(ascii, length(min = 0, max = 400))] pub String);

impl Default for About {
  fn default() -> Self { About("about ipsum dolor".to_string()) }
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
  fn default() -> Self { Username("alice".to_string()) }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Type)]
#[garde(transparent)]
#[repr(transparent)]
pub struct Email(#[garde(email)] pub String);

impl Default for Email {
  fn default() -> Self { Email("email@email.com".to_string()) }
}

impl std::fmt::Display for Email {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}

/// A raw, unhashed password
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[garde(transparent)]
pub struct Password(#[garde(ascii, length(min = 8, max = 25))] pub String);
impl Password {
  pub async fn hash_argon(&self) -> DbResult<PasswordHash> { hash_password_argon(self).await }
}

impl Default for Password {
  fn default() -> Self {
    warn!("instantiating the insecure default password");
    Password("password".to_string())
  }
}

/// A hashed password
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[repr(transparent)]
pub struct PasswordHash(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[repr(transparent)]
pub struct AuthToken(pub String);
// pub struct AuthToken(pub oauth2::CsrfToken);

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[repr(transparent)]
pub struct ResetPasswordToken(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Type, Validate)]
#[garde(transparent)]
#[repr(transparent)]
pub struct Title(#[garde(ascii, length(min = 8, max = 100))] pub String);

impl Default for Title {
  fn default() -> Self { Title("title".to_string()) }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, Validate)]
#[garde(transparent)]
#[repr(transparent)]
pub struct CommentText(#[garde(ascii, length(min = 8, max = 2000))] pub String);

impl Default for CommentText {
  fn default() -> Self { CommentText("comment ipsum dolor".to_string()) }
}
