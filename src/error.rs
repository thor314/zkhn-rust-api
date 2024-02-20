//! zkhn-rust-api error types
// https://docs.rs/thiserror/latest/thiserror/

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
  // Derive Into<MyError> for io errors
  #[error("My Io error: {0}")]
  Io(#[from] std::io::Error),
  // Derive Into<MyError> for anyhow errors
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
  #[error("My Password error: {0}")]
  PwError(#[from] PasswordError),
  // Some other error type
  #[allow(dead_code)]
  #[error("an unhandled error")]
  Unhandled,
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
  #[error("bcrypt error: {0}")]
  BcryptError(#[from] bcrypt::BcryptError),
  #[error("passwords do not match")]
  PasswordMismatch,
}