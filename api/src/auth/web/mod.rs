use axum::{
  extract::Query,
  http::StatusCode,
  response::{IntoResponse, Redirect},
  routing::{get, post},
  Form, Json,
};
use serde::Deserialize;

use crate::{
  auth::users::{AuthSession, CredentialsPayload},
  ApiError, ApiResult,
};

/// Internal login logic.
/// Isolate from the login handler to maintain consistency with axum-login style example.
pub async fn login_post_internal(
  mut auth_session: AuthSession,
  creds: CredentialsPayload,
) -> ApiResult<Redirect> {
  // verify the user's login credentials
  let user = match auth_session.authenticate(creds.clone()).await {
    // login success
    Ok(Some(user)) => user,
    // login incorrect password
    Ok(None) => {
      let login_base_url = "/users/login";
      let login_url = creds
        .next
        .map(|next| format!("{}?next={}", login_base_url, next))
        .unwrap_or(login_base_url.to_string());

      return Ok(Redirect::to(&login_url));
    },
    // internal authentication error
    Err(_) => return Err(ApiError::OtherISE("authentication logic error".to_string())),
  };

  // update the session with the user's login
  if auth_session.login(&user).await.is_err() {
    // internal authentication error
    return Err(ApiError::OtherISE("authentication login logic error".to_string()));
  }

  let response = creds.next.map(|next| Redirect::to(&next)).unwrap_or_else(|| Redirect::to("/"));
  Ok(response)
}

/// Internal logout logic.
/// Isolate from the login handler to maintain consistency with axum-login style example.
pub async fn logout_post_internal(mut auth_session: AuthSession) -> ApiResult<Redirect> {
  match auth_session.logout().await {
    Ok(_) => Ok(Redirect::to("/users/login")),
    Err(_) => Err(ApiError::OtherISE("logout error".to_string())),
  }
}
