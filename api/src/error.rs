//! zkhn-rust-api error types
// https://docs.rs/thiserror/latest/thiserror/

use axum::{
  http::{status, StatusCode},
  response::IntoResponse,
};
use db::DbError;
use oauth2::{
  basic::{BasicClient, BasicRequestTokenError},
  reqwest::{async_http_client, AsyncHttpClientError},
  url::Url,
  AuthorizationCode, CsrfToken, TokenResponse,
};
use serde::Serialize;
use tokio::task;

#[derive(thiserror::Error, axum_derive_error::ErrorResponse)]
pub enum ApiError {
  // 500s
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  OtherISE(String),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  TaskJoin(#[from] task::JoinError),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  Anyhow(#[from] anyhow::Error),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  DbError(#[from] DbError),
  #[status(status::StatusCode::INTERNAL_SERVER_ERROR)]
  Session(tower_sessions::session_store::Error),
  #[status(StatusCode::BAD_REQUEST)]
  AuthenticationError(String),
  // 400s
  #[status(StatusCode::NOT_FOUND)]
  DbEntryNotFound(String),
  #[status(StatusCode::CONFLICT)]
  DbEntryAlreadyExists(String),
  #[status(StatusCode::UNAUTHORIZED)]
  Unauthorized(String),
  /// for when e.g. an upvote or favorite is doubly submitted
  #[status(StatusCode::BAD_REQUEST)]
  DoublySubmittedChange(String),
  #[status(StatusCode::BAD_REQUEST)]
  Payload(String),
  #[status(StatusCode::BAD_REQUEST)]
  GardePayload(#[from] garde::Report),
  #[status(StatusCode::UNAUTHORIZED)]
  PwError(String),
  #[status(StatusCode::UNAUTHORIZED)]
  AuthReqwest(#[from] reqwest::Error),
  #[status(StatusCode::UNAUTHORIZED)]
  OAuth2(BasicRequestTokenError<AsyncHttpClientError>),
  #[status(StatusCode::NOT_FOUND)]
  MissingField(String),
  // don't uncomment - creates circular dependency
  // #[status(StatusCode::UNAUTHORIZED)]
  // AxumLogin(#[from] axum_login::Error<crate::auth::Backend>),
}

impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ApiError::OtherISE(e) => write!(f, "OtherISE: {0}", e),
      ApiError::TaskJoin(e) => write!(f, "TaskJoin: {0}", e),
      ApiError::Anyhow(e) => write!(f, "Anyhow: {0}", e),
      ApiError::PwError(e) => write!(f, "PwError: {0}", e),
      ApiError::DbError(e) => write!(f, "DbError: {0}", e),
      ApiError::Session(e) => write!(f, "Session: {0}", e),
      ApiError::Payload(e) => write!(f, "Payload {0}", e),
      ApiError::DbEntryNotFound(e) => write!(f, "NotFound: {0}", e),
      ApiError::Unauthorized(e) => write!(f, "Unauthorized: {0}", e),
      ApiError::GardePayload(e) => write!(f, "GardePayload: {0}", e),
      ApiError::DbEntryAlreadyExists(e) => write!(f, "DbEntryAlreadyExists: {0}", e),
      ApiError::AuthenticationError(e) => write!(f, "AuthenticationError: {0}", e),
      ApiError::AuthReqwest(e) => write!(f, "AuthReqwest: {0}", e),
      ApiError::OAuth2(e) => write!(f, "OAuth2: {0}", e),
      ApiError::DoublySubmittedChange(e) => write!(f, "DoublySubmittedChange: {0}", e),
      ApiError::MissingField(e) => write!(f, "MissingField: {0}", e),
    }
  }
}
