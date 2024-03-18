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
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  TaskJoin(#[from] task::JoinError),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  Anyhow(#[from] anyhow::Error),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  PwError(#[from] PasswordError),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  DbError(#[from] DbError),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  Session(tower_sessions::session_store::Error),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  Payload(#[from] PayloadError),
  Route(#[from] RouteError),
}

impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ApiError::TaskJoin(e) => write!(f, "TaskJoin: {0}", e),
      ApiError::Anyhow(e) => write!(f, "Anyhow: {0}", e),
      ApiError::PwError(e) => write!(f, "PwError: {0}", e),
      ApiError::DbError(e) => write!(f, "DbError: {0}", e),
      ApiError::Session(e) => write!(f, "Session: {0}", e),
      ApiError::Payload(e) => write!(f, "Payload: {0}", e),
      ApiError::Route(e) => write!(f, "Route: {0}", e),
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
