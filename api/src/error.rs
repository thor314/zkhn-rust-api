//! zkhn-rust-api error types
// https://docs.rs/thiserror/latest/thiserror/

use db::error::DbError;
use thiserror::Error;
use tokio::task;

#[derive(Debug, Error)]
pub enum ApiError {
  #[error(transparent)]
  TaskJoin(#[from] task::JoinError),
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
  #[error(transparent)]
  PwError(#[from] PasswordError),
  #[error(transparent)]
  DbError(#[from] DbError),
  #[error(transparent)]
  Session(#[from] tower_sessions::session_store::Error),
  #[allow(dead_code)]
  #[error("an unhandled error")]
  Unhandled,
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
  #[error("scrypt error: {0}")]
  ScryptPwHashError(#[from] scrypt::password_hash::Error),
}
