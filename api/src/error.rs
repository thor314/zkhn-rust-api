//! zkhn-rust-api error types
// https://docs.rs/thiserror/latest/thiserror/

use diesel_async::pooled_connection::deadpool::PoolError;
use thiserror::Error;
use tokio::task;

#[derive(Debug, Error)]
pub enum ApiError {
  #[error(transparent)]
  TaskJoin(#[from] task::JoinError),
  #[error("My Io error: {0}")]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
  #[error("My Password error: {0}")]
  PwError(#[from] PasswordError),
  #[error("Diesel error: {0}")]
  Diesel(#[from] diesel::result::Error),
  #[error("deadpool error: {0}")]
  Deadpool(#[from] PoolError),
  #[allow(dead_code)]
  #[error("an unhandled error")]
  Unhandled,
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
  #[error("scrypt error: {0}")]
  ScryptPwHashError(#[from] scrypt::password_hash::Error),
  // #[error("failed to hash password, do not match")]
  // PasswordMismatch,
}
