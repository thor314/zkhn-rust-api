use axum::{
  http::StatusCode, response::{IntoResponse, Redirect}, Form, Json, Router
};
use tracing::info;

use crate::{error::ApiError, ApiResult};

use super::{credentials::Credentials, AuthSession};

pub async fn login_handler(
  mut auth_session: AuthSession,
  Json(creds): Json<Credentials>,
) -> ApiResult<impl IntoResponse> {
  // dbg!("auth: {:?}", &auth_session);
  // dbg!("creds: {:?}", &creds);
  let user = match auth_session.authenticate(creds.clone()).await {
    Ok(Some(user)) => user,
    Ok(None) => return Err(ApiError::DbEntryNotFound("Auth: No such user found".to_string())),
    Err(e) => return Err(ApiError::AuthenticationError(e.to_string())),
  };
  // dbg!("user: {:?}", &user);

  if let Err(e) = auth_session.login(&user).await {
    return Err(ApiError::AuthenticationError(e.to_string()))
  }

  // todo: redirect to the "next" field in the query string or credentials struct
  Ok(Redirect::to("/protected").into_response())
}

pub async fn logout_handler(
  mut auth_session: AuthSession,
) -> impl IntoResponse {
  dbg!("auth: {:?}", &auth_session);
  auth_session.logout().await.unwrap(); // todo: unwrap fix
  StatusCode::OK
}
