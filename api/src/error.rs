//! zkhn-rust-api error types
// https://docs.rs/thiserror/latest/thiserror/

use std::f32::consts::E;

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
use utoipa::ToSchema;

// ref: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
#[derive(thiserror::Error, axum_derive_error::ErrorResponse, ToSchema)]
pub enum ApiError {
  // 500s
  #[status(StatusCode::INTERNAL_SERVER_ERROR)]
  OtherISE(String),
  #[status(StatusCode::INTERNAL_SERVER_ERROR)]
  TaskJoin(#[from] task::JoinError),
  #[status(StatusCode::INTERNAL_SERVER_ERROR)]
  Anyhow(#[from] anyhow::Error),
  #[status(StatusCode::INTERNAL_SERVER_ERROR)]
  Session(tower_sessions::session_store::Error),

  /// OAuth API service is temporarily unavailable due to maintenance, overload, or other reasons
  #[status(StatusCode::SERVICE_UNAVAILABLE)] // 503
  OAuthRequestFailure(#[from] reqwest::Error),
  /// received an invalid response from the OAuth server
  #[status(StatusCode::BAD_GATEWAY)] // 503
  OAuthBadGateway(String),

  // Db errors
  #[status(StatusCode::NOT_FOUND)]
  DbEntryNotFound(String),
  #[status(StatusCode::CONFLICT)]
  DbConflict(String),
  #[status(StatusCode::INTERNAL_SERVER_ERROR)]
  OtherDbError(String),

  // 400s
  #[status(StatusCode::BAD_REQUEST)] // 400
  BadRequest(String),
  #[status(StatusCode::BAD_REQUEST)] // 400
  OAuth2(BasicRequestTokenError<AsyncHttpClientError>),
  #[status(StatusCode::UNAUTHORIZED)] // 401
  Unauthorized(String),
  #[status(StatusCode::UNAUTHORIZED)] // 401
  IncorrectPassword(String),
  /// for when a required field is missing in the table
  #[status(StatusCode::NOT_FOUND)] // 404
  MissingField(String),
  /// for when e.g. an upvote or favorite is doubly submitted
  #[status(StatusCode::CONFLICT)] // 409
  DoublySubmittedChange(String),
  #[status(StatusCode::UNPROCESSABLE_ENTITY)]
  InvalidPayload(#[from] garde::Report), /* 422
                                          * don't uncomment - creates circular dependency
                                          * #[status(StatusCode::UNAUTHORIZED)]
                                          * AxumLogin(#[from]
                                          * axum_login::Error<crate::auth::Backend>), */
}

impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ApiError::OtherISE(e) => write!(f, "OtherISE: {0}", e),
      ApiError::TaskJoin(e) => write!(f, "TaskJoin: {0}", e),
      ApiError::Anyhow(e) => write!(f, "Anyhow: {0}", e),
      ApiError::IncorrectPassword(e) => write!(f, "Incorrect Password: {0}", e),
      ApiError::OtherDbError(e) => write!(f, "DbError: {0}", e),
      ApiError::Session(e) => write!(f, "Session: {0}", e),
      // ApiError::Payload(e) => write!(f, "Payload {0}", e),
      ApiError::DbEntryNotFound(e) => write!(f, "NotFound: {0}", e),
      ApiError::Unauthorized(e) => write!(f, "Unauthorized: {0}", e),
      ApiError::InvalidPayload(e) => write!(f, "Invalid Payload: {0}", e.to_string().trim()),
      ApiError::DbConflict(e) => write!(f, "DbEntryAlreadyExists: {0}", e),
      ApiError::BadRequest(e) => write!(f, "Invalid request submitted: {0}", e),
      ApiError::OAuthRequestFailure(e) => write!(f, "AuthReqwest: {0}", e),
      ApiError::OAuth2(e) => write!(f, "OAuth2: {0}", e),
      ApiError::DoublySubmittedChange(e) => write!(f, "DoublySubmittedChange: {0}", e),
      ApiError::MissingField(e) => write!(f, "MissingField: {0}", e),
      ApiError::OAuthBadGateway(e) => write!(f, "OAuthBadGateway: {0}", e),
    }
  }
}

impl From<DbError> for ApiError {
  fn from(e: DbError) -> Self {
    match e {
      DbError::Conflict => ApiError::DbConflict(e.to_string()),
      DbError::NotFound => ApiError::DbEntryNotFound(e.to_string()),
      // keep PwError commented - db password error is internal library failure
      // DbError::PwError(e) => ApiError::IncorrectPassword(e.to_string()),
      // DbError::PayloadValidation(e) => ApiError::(e.to_string()),
      _ => ApiError::OtherDbError(e.to_string()),
    }
  }
}
