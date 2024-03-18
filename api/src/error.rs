//! zkhn-rust-api error types
// https://docs.rs/thiserror/latest/thiserror/

use axum::{
  http::{status, StatusCode},
  response::IntoResponse,
};
use db::DbError;
use tokio::task;

#[derive(thiserror::Error, axum_derive_error::ErrorResponse)]
pub enum ApiError {
  // #[error(transparent)]
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  TaskJoin(#[from] task::JoinError),
  // #[error(transparent)]
  // #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  // Io(std::io::Error),
  // #[error(transparent)]
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  Anyhow(#[from] anyhow::Error),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  // #[error(transparent)]
  PwError(#[from] PasswordError),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  // #[error(transparent)]
  DbError(#[from] DbError),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  // #[error(transparent)]
  Session(tower_sessions::session_store::Error),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  // #[error(transparent)]
  Payload(#[from] PayloadError),
  // #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  // #[error(transparent)]
  Route(#[from] RouteError),
  // #[allow(dead_code)]
  // #[error("an unhandled error")]
  // Unhandled,
}

impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ApiError::TaskJoin(_) => todo!(),
      ApiError::Anyhow(_) => todo!(),
      ApiError::PwError(_) => todo!(),
      ApiError::DbError(_) => todo!(),
      ApiError::Session(_) => todo!(),
      ApiError::Payload(_) => todo!(),
      ApiError::Route(_) => todo!(),
    }
  }
}

#[derive(thiserror::Error, axum_derive_error::ErrorResponse)]
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

#[derive(Debug, thiserror::Error)]
pub enum PayloadError {
  #[error("error converting payload to user: {0}")]
  UserTryFromError(#[from] anyhow::Error),
}
