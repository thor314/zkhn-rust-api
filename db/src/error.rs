//! zkhn-rust-api error types
// https://docs.rs/thiserror/latest/thiserror/

use std::{error, fmt::Display};

use sqlx::{error::ErrorKind, migrate::MigrateError};
use thiserror::Error;
use tokio::task;

#[derive(Debug, Error)]
pub enum DbError {
  // /// task concurrency error
  // #[error(transparent)]
  // TaskJoin(#[from] task::JoinError),
  /// Catch all sqlx query error
  #[error("Sqlx error: {0}")]
  Sqlx(String),

  // database errors
  #[error("Entry already exists in db: {0}")]
  UniqueViolation(String),
  #[error("Foreign Key Violation in db: {0}")]
  ForeignKeyViolation(String),
  #[error("Not Null Violation in db: {0}")]
  NotNullViolation(String),
  #[error("Check Violation in db: {0}")]
  CheckViolation(String),
  #[error("Other db error: {0}")]
  Other(String),

  #[error("Entry not found in db: {0}")]
  NotFound(String),

  #[error("Entry not editable: {0}")]
  NotEditable(String),
}

impl From<sqlx::Error> for DbError {
  fn from(err: sqlx::Error) -> Self {
    match err {
      sqlx::Error::Database(db_err) => match db_err.kind() {
        ErrorKind::UniqueViolation => DbError::UniqueViolation(db_err.to_string()),
        ErrorKind::ForeignKeyViolation => DbError::ForeignKeyViolation(db_err.to_string()),
        ErrorKind::NotNullViolation => DbError::NotNullViolation(db_err.to_string()),
        ErrorKind::CheckViolation => DbError::CheckViolation(db_err.to_string()),
        ErrorKind::Other => DbError::Other(db_err.to_string()),
        _ => DbError::Sqlx(db_err.to_string()),
      },
      _ => DbError::Sqlx(err.to_string()),
    }
  }
}
