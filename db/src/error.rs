//! zkhn-rust-api error types
// https://docs.rs/thiserror/latest/thiserror/

use sqlx::migrate::MigrateError;
use thiserror::Error;
use tokio::task;

#[derive(Debug, Error)]
pub enum DbError {
  #[error(transparent)]
  TaskJoin(#[from] task::JoinError),
  #[error(transparent)]
  Sqlx(#[from] sqlx::Error),
  #[error(transparent)]
  SqlxMigrate(#[from] MigrateError),
  #[error("Not found in database")]
  NotFound,
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
  // todo remove pw error  move to api
  #[error("Invalid favorite state encountered")]
  InvalidFavoriteState,
  #[error(transparent)]
  PwError(#[from] scrypt::password_hash::Error),
  #[error(transparent)]
  Recoverable(#[from] RecoverableDbError),
}

#[derive(Debug, Error)]
pub enum RecoverableDbError {
  #[error("Entry already exists")]
  DbEntryAlreadyExists,
}
