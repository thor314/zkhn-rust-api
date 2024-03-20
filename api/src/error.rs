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
  // 500s
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  TaskJoin(#[from] task::JoinError),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  Anyhow(#[from] anyhow::Error),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  DbError(#[from] DbError),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  Session(tower_sessions::session_store::Error),
  // 400s
  #[status(StatusCode::NOT_FOUND)]
  DbEntryNotFound(String),
  #[status(StatusCode::UNAUTHORIZED)]
  Unauthorized(String),
  #[status(StatusCode::BAD_REQUEST)]
  Payload(String),
  #[status(StatusCode::BAD_REQUEST)]
  GardePayload(#[from] garde::Report),
  #[status(StatusCode::UNAUTHORIZED)]
  PwError(String),
}

impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ApiError::TaskJoin(e) => write!(f, "TaskJoin: {0}", e),
      ApiError::Anyhow(e) => write!(f, "Anyhow: {0}", e),
      ApiError::PwError(e) => write!(f, "PwError: {0}", e),
      ApiError::DbError(e) => write!(f, "DbError: {0}", e),
      ApiError::Session(e) => write!(f, "Session: {0}", e),
      ApiError::Payload(e) => write!(f, "Payload {0}", e),
      ApiError::DbEntryNotFound(e) => write!(f, "NotFound: {0}", e),
      ApiError::Unauthorized(e) => write!(f, "Unauthorized: {0}", e),
      ApiError::GardePayload(e) => write!(f, "GardePayload: {0}", e),
    }
  }
}
