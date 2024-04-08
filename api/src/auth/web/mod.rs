use axum::http::StatusCode;
use tracing::{debug, error};

use crate::{auth::users::AuthSession, ApiError, ApiResult, CredentialsPayload};

/// Internal login logic.
///
/// Isolate from the login handler to maintain consistency with axum-login style example.
pub async fn login_post_internal(
  mut auth_session: AuthSession,
  creds: CredentialsPayload,
) -> ApiResult<StatusCode> {
  // safety - authenticate never returns None
  let user = auth_session.authenticate(creds.clone()).await?.unwrap();
  auth_session.login(&user).await?;
  debug!("login success for user: {}", creds.username);
  Ok(StatusCode::OK)
}

/// Internal logout logic.
///
/// Isolate from the login handler to maintain consistency with axum-login style example.
/// Will only error if db fails to flush the session.
pub async fn logout_post_internal(mut auth_session: AuthSession) -> ApiResult<StatusCode> {
  // NB - logout may return None if the user is not already logged in, ignored
  auth_session.logout().await?;
  Ok(StatusCode::OK)
}
