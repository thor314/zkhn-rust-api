use axum::{
  http::StatusCode,
  response::{IntoResponse, Redirect},
  routing::{get, post},
  Form, Json, Router,
};
use tower_sessions::Session;
use tracing::{debug, info};

use super::{
  credentials::{password_creds::PasswordCreds, Credentials},
  AuthSession,
};
use crate::{error::ApiError, ApiResult};

/// todo move to oauth
pub const CSRF_STATE_KEY: &str = "oauth.csrf-state";
/// todo
pub const NEXT_URL_KEY: &str = "aochuracrhkeo";

pub fn auth_router() -> Router {
  Router::new()
    .route("/login/password", post(login_password))
    .route("/login/oauth", post(login_oauth))
    .route("/logout", post(logout_handler))
}

// ref: https://github.com/maxcountryman/axum-login/blob/main/examples/multi-auth/src/web/auth.rs#L45
// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L48
/// WIP: need to add logic to mirror js handler
/// BLOCKED: https://github.com/maxcountryman/axum-login/pull/210
async fn login_password(
  mut auth_session: AuthSession,
  Json(creds): Json<PasswordCreds>,
) -> ApiResult<impl IntoResponse> {
  let user = match auth_session.authenticate(Credentials::Password(creds.clone())).await {
    Ok(Some(user)) => user,
    Ok(None) => return Err(ApiError::AuthenticationError("Invalid credentials.".to_string())),
    Err(e) => return Err(ApiError::OtherISE(e.to_string())),
  };

  if let Err(e) = auth_session.login(&user).await {
    // todo: breakpoint
    tracing::error!("login error: {}", e);
    return Err(ApiError::AuthenticationError(e.to_string()));
  }

  // todo: check if user is banned

  // todo: redirect to the "next" field in the query string or credentials struct
  info!("logged in user: {}", user.0.username);
  let redirect_location = creds.next.clone().unwrap_or_else(|| "/".to_string());
  Ok(Redirect::to(&redirect_location).into_response())
  // todo: return user data
}

// ref: https://github.com/maxcountryman/axum-login/blob/main/examples/multi-auth/src/web/auth.rs#L75
async fn login_oauth(
  mut auth_session: AuthSession,
  session: Session,
  // Json(creds): Json,
) -> ApiResult<impl IntoResponse> {
  let (auth_url, csrf_state) = auth_session.backend.authorize_url();

  session
    .insert(CSRF_STATE_KEY, csrf_state.secret())
    .await
    .expect("Serialization should not fail.");

  // session.insert(NEXT_URL_KEY, next).await.expect("Serialization should not fail.");

  Ok(()) // todo!()
         // dbg!("auth: {:?}", &auth_session);
         // dbg!("creds: {:?}", &creds);
         // let user = match auth_session.authenticate(creds.clone()).await {
         //   Ok(Some(user)) => user,
         //   Ok(None) => return Err(ApiError::DbEntryNotFound("Auth: No such user
         // found".to_string())),   Err(e) => return
         // Err(ApiError::AuthenticationError(e.to_string())), };
         // // dbg!("user: {:?}", &user);

  // if let Err(e) = auth_session.login(&user).await {
  //   return Err(ApiError::AuthenticationError(e.to_string()));
  // }

  // // todo: redirect to the "next" field in the query string or credentials struct
  // Ok(Redirect::to("/protected").into_response())
}

// todo: blocked
// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L123
async fn logout_handler(mut auth_session: AuthSession) -> ApiResult<StatusCode> {
  dbg!("auth: {:?}", &auth_session);
  auth_session.logout().await.map_err(|e| ApiError::AuthenticationError(e.to_string()))?;
  Ok(StatusCode::OK)
}
