//! zkhn-rust-api error types
// https://docs.rs/thiserror/latest/thiserror/

use axum::http::StatusCode;
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
  #[error(transparent)]
  Route(#[from] RouteError),
  #[allow(dead_code)]
  #[error("an unhandled error")]
  Unhandled,
}

#[derive(Error, axum_derive_error::ErrorResponse)]
pub enum RouteError {
  #[status(StatusCode::NOT_FOUND)]
  NotFound,
  #[status(StatusCode::UNAUTHORIZED)]
  Unauthorized,
  #[status(StatusCode::BAD_REQUEST)]
  BadRequest,
}

impl std::fmt::Display for RouteError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      RouteError::NotFound => write!(f, "Not Found"),
      RouteError::Unauthorized => write!(f, "Unauthorized"),
        RouteError::BadRequest => write!(f, "Bad Request"),
    }
  }
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
  #[error("scrypt error: {0}")]
  ScryptPwHashError(#[from] scrypt::password_hash::Error),
}
