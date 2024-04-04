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
  UnauthorizedPleaseLogin,
  /// The client submitted an incorrect password
  #[status(StatusCode::UNAUTHORIZED)] // 401
  UnauthorizedIncorrectPassword,
  /// The client submitted an incorrect password
  #[status(StatusCode::FORBIDDEN)] // 403
  ForbiddenUsernameDoesNotMatchSession,
  /// The user is banned
  #[status(StatusCode::FORBIDDEN)] // 401
  ForbiddenBanned,
  /// Caller must be a moderator
  #[status(StatusCode::FORBIDDEN)] // 403
  ForbiddenModeratorRequired,
  /// Garde payload validation failure.
  #[status(StatusCode::UNPROCESSABLE_ENTITY)] // 422
  InvalidPayload(#[from] garde::Report),
}

impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ApiError::OtherISE(e) => write!(f, "Thor did a bad thing, ISE: {e}"),
      ApiError::TaskJoin(e) => write!(f, "Concurrency Error: {e}"),
      // db errors
      ApiError::UniqueViolation(e) => write!(f, "DbEntryAlreadyExists: {e}"),
      ApiError::ForeignKeyViolation(e) => write!(f, "DbForeignKeyViolation: {e}"),
      ApiError::NotNullViolation(e) => write!(f, "DbNotNullViolation: {e}"),
      ApiError::CheckViolation(e) => write!(f, "DbCheckViolation: {e}"),
      ApiError::DbEntryNotFound(e) => write!(f, "Not Found: {e}"),
      ApiError::OtherDbError(e) => write!(f, "DbError: {e}"),
      // other client errors
      ApiError::BadRequest(e) => write!(f, "Invalid request submitted: {e}"),
      ApiError::UnauthorizedPleaseLogin => write!(f, "Unauthorized: please log in",),
      ApiError::UnauthorizedIncorrectPassword => write!(f, "Unauthorized: Incorrect password"),
      ApiError::ForbiddenBanned => write!(f, "Unauthorized: User is banned"),
      ApiError::ForbiddenUsernameDoesNotMatchSession =>
        write!(f, "Forbidden: provided username does not match session"),
      ApiError::ForbiddenModeratorRequired => write!(f, "Forbidden: Moderator only"),
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
      DbError::NotFound(e) => ApiError::DbEntryNotFound(e),
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
