//! zkhn-rust-api error types
// https://docs.rs/thiserror/latest/thiserror/

use std::fmt::Display;

use sqlx::migrate::MigrateError;
use thiserror::Error;
use tokio::task;

#[derive(Debug, Error)]
pub enum DbError {
  // developer error (the bad news)
  #[error(transparent)]
  TaskJoin(#[from] task::JoinError),
  #[error(transparent)]
  Sqlx(#[from] sqlx::Error),
  #[error(transparent)]
  SqlxMigrate(#[from] MigrateError),
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),

  // user error
  /// Library error, i.e. Hashing failed (hope to catch in password validation stage)
  #[error(transparent)]
  PwError(#[from] scrypt::password_hash::Error),
  #[error("Entry already exists")]
  Conflict,
  #[error("Entry not found in database")]
  NotFound,
}
