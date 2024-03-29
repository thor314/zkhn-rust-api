use axum::{
  extract::Query,
  http::StatusCode,
  response::{IntoResponse, Redirect},
  routing::{get, post},
  Form, Json,
};
use serde::Deserialize;
use tracing::debug;

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
    // login incorrect password - reroute to login page
    Ok(None) => {
      let login_base_url = "/users/login";
      let login_url = creds
        .next
        .map(|next| format!("{}?next={}", login_base_url, next))
        .unwrap_or(login_base_url.to_string());

      debug!("login incorrect password for user: {}", creds.username);
      return Ok(Redirect::to(&login_url));
    },
    // internal authentication error
    Err(_) => return Err(ApiError::OtherISE("authentication logic error".to_string())),
  };

  // update the session with the user's login
  if auth_session.login(&user).await.is_err() {
    // internal authentication error
    debug!("login ise for user: {}", creds.username);
    return Err(ApiError::OtherISE("authentication login logic error".to_string()));
  }

  let redirect_uri = creds.next.unwrap_or("/".to_string());
  debug!("login success for user: {}, redirecting to {}", creds.username, redirect_uri);
  Ok(Redirect::to(&redirect_uri))
}

/// Internal logout logic.
/// Isolate from the login handler to maintain consistency with axum-login style example.
pub async fn logout_post_internal(mut auth_session: AuthSession) -> ApiResult<Redirect> {
  match auth_session.logout().await {
    Ok(_r) => Ok(Redirect::to("/users/login")),
    Err(_e) => Err(ApiError::OtherISE("logout error: {_e:?}".to_string())),
  }
}
