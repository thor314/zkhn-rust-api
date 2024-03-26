//! Newtype wrappers for input validation and type-safety
use std::fmt;

use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, Decode, Encode};
use uuid::Uuid;

use crate::{password::hash_password, DbResult};

/// A timestamp wrapper that we can use in our sqlx models.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Timestamp(pub DateTime<Utc>);

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Type)]
#[garde(transparent)]
#[repr(transparent)]
pub struct About(#[garde(ascii, length(min = 0, max = 400))] pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq, Type)]
#[garde(transparent)]
#[repr(transparent)]
pub struct Username(#[garde(ascii, length(min = 3, max = 25))] pub String);

impl fmt::Display for Username {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Type)]
#[garde(transparent)]
#[repr(transparent)]
pub struct Email(#[garde(email)] pub String);

impl std::fmt::Display for Email {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}

/// A raw, unhashed password
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[garde(transparent)]
pub struct Password(#[garde(ascii, length(min = 8, max = 25))] pub String);
impl Password {
  pub fn hash(&self) -> DbResult<PasswordHash> { hash_password(self) }
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

#[derive(Debug, Clone, Serialize, Deserialize, Type, Validate)]
#[garde(transparent)]
#[repr(transparent)]
pub struct CommentText(#[garde(ascii, length(min = 8, max = 2000))] pub String);
