//! zkhn-rust-api error types
// https://docs.rs/thiserror/latest/thiserror/

use std::fmt::write;

use axum::http::StatusCode;
use db::DbError;
use oauth2::{basic::BasicRequestTokenError, reqwest::AsyncHttpClientError};
use tokio::task;
use utoipa::ToSchema;

use crate::auth::AuthBackend;

// ref: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
// note: use 401 Unauthorized if the client is unknown,
// and 403 Forbidden if the client is known but privilege restricted.
#[derive(thiserror::Error, axum_derive_error::ErrorResponse, ToSchema)]
pub enum ApiError {
  // return a 500 when my backend screwed up
  /// General error to return when I'm not sure what went wrong
  #[status(StatusCode::INTERNAL_SERVER_ERROR)]
  OtherISE(String),
  /// Merge concurrent tasks error
  #[status(StatusCode::INTERNAL_SERVER_ERROR)]
  TaskJoin(#[from] task::JoinError),

  // db errors
  /// New entry conflicts with another entry in the db
  #[status(StatusCode::CONFLICT)] // 409
  UniqueViolation(String),
  #[status(StatusCode::INTERNAL_SERVER_ERROR)]
  ForeignKeyViolation(String),
  #[status(StatusCode::INTERNAL_SERVER_ERROR)]
  NotNullViolation(String),
  #[status(StatusCode::INTERNAL_SERVER_ERROR)]
  CheckViolation(String),
  /// Entry does not exist in the db
  #[status(StatusCode::NOT_FOUND)] // 404
  DbEntryNotFound(String),
  #[status(StatusCode::INTERNAL_SERVER_ERROR)]
  OtherDbError(String),

  // other client errors
  /// The server cannot or will not process the request due to  client error
  /// (malformed request syntax, invalid request message framing, deceptive request routing, ...).
  #[status(StatusCode::BAD_REQUEST)] // 400
  BadRequest(String),
  /// The client must authenticate itself to get the requested response
  #[status(StatusCode::UNAUTHORIZED)] // 401
  Unauthorized(String),
  /// The client does not have access rights to the content; that is, it is unauthorized, so the
  /// server is refusing to give the requested resource.
  #[status(StatusCode::FORBIDDEN)] // 403
  Forbidden(String),
  /// Caller must be a moderator
  #[status(StatusCode::FORBIDDEN)] // 403
  ForbiddenModeratorRequired(String),
  /// Garde payload validation failure.
  #[status(StatusCode::UNPROCESSABLE_ENTITY)] // 422
  InvalidPayload(#[from] garde::Report),
}

impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ApiError::OtherISE(e) => write!(f, "Thor did a bad thing, ISE: {0}", e),
      ApiError::TaskJoin(e) => write!(f, "Concurrency Error: {0}", e),
      // db errors
      ApiError::UniqueViolation(e) => write!(f, "DbEntryAlreadyExists: {0}", e),
      ApiError::ForeignKeyViolation(e) => write!(f, "DbForeignKeyViolation: {0}", e),
      ApiError::NotNullViolation(e) => write!(f, "DbNotNullViolation: {0}", e),
      ApiError::CheckViolation(e) => write!(f, "DbCheckViolation: {0}", e),
      ApiError::DbEntryNotFound(e) => write!(f, "NotFound: {0}", e),
      ApiError::OtherDbError(e) => write!(f, "DbError: {0}", e),
      // other client errors
      ApiError::BadRequest(e) => write!(f, "Invalid request submitted: {0}", e),
      ApiError::Unauthorized(e) => write!(f, "Unauthorized: {0}", e),
      ApiError::Forbidden(e) => write!(f, "Forbidden: {0}", e),
      ApiError::ForbiddenModeratorRequired(e) => write!(f, "Forbidden, Moderator only: {0}", e),
      ApiError::InvalidPayload(e) => write!(f, "Invalid Payload: {0}", e.to_string().trim()),
    }
  }
}

impl From<DbError> for ApiError {
  fn from(e: DbError) -> Self {
    match e {
      DbError::UniqueViolation(e) => ApiError::UniqueViolation(e),
      DbError::ForeignKeyViolation(e) => ApiError::ForeignKeyViolation(e),
      DbError::NotNullViolation(e) => ApiError::NotNullViolation(e),
      DbError::CheckViolation(e) => ApiError::CheckViolation(e),
      DbError::NotFound => ApiError::DbEntryNotFound("Entry not found".to_string()),
      DbError::Other(e) => ApiError::OtherDbError(e),
      _ => ApiError::OtherDbError(e.to_string()),
    }
  }
}

impl From<axum_login::Error<AuthBackend>> for ApiError {
  fn from(e: axum_login::Error<AuthBackend>) -> Self {
    match e {
      axum_login::Error::Session(e) =>
        ApiError::OtherISE("Unknown axum_login session error".to_string()),
      axum_login::Error::Backend(e) => e,
    }
  }
}

// /// OAuth API service is temporarily unavailable due to maintenance, overload, or other reasons
// #[status(StatusCode::SERVICE_UNAVAILABLE)] // 503
// OAuthRequestFailure(#[from] reqwest::Error),
// /// received an invalid response from the OAuth server
// #[status(StatusCode::BAD_GATEWAY)] // 503
// OAuthBadGateway(String),

// don't uncomment - creates circular dependency
// #[status(StatusCode::UNAUTHORIZED)]
// AxumLogin(#[from]
// axum_login::Error<crate::auth::Backend>),

// #[status(StatusCode::BAD_REQUEST)] // 400
// OAuth2(BasicRequestTokenError<AsyncHttpClientError>),
