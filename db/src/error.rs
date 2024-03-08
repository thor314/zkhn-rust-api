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
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
  #[error(transparent)]
  PwError(#[from] PasswordError),
  #[error("Invalid favorite state encountered")]
  InvalidFavoriteState,
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
  #[error(transparent)]
  ScryptPwHashError(#[from] scrypt::password_hash::Error),
}
