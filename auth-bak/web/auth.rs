use axum::{
  extract::Query,
  http::StatusCode,
  response::{IntoResponse, Redirect},
  routing::{get, post},
  Form, Json,
};
use serde::Deserialize;

use crate::auth::users::{AuthSession, CredentialsPayload};

// This allows us to extract the "next" field from the query string. We use this
// to redirect after log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
  next: Option<String>,
}

pub async fn login_post(
  mut auth_session: AuthSession,
  Json(creds): Json<CredentialsPayload>,
) -> impl IntoResponse {
  let user = match auth_session.authenticate(creds.clone()).await {
    Ok(Some(user)) => user,
    Ok(None) => {
      let mut login_url = "/login".to_string();
      if let Some(next) = creds.next {
        login_url = format!("{}?next={}", login_url, next);
      };

      return Redirect::to(&login_url).into_response();
    },
    Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  };

  if auth_session.login(&user).await.is_err() {
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }

  if let Some(ref next) = creds.next { Redirect::to(next) } else { Redirect::to("/") }
    .into_response()
}

pub async fn logout_get(mut auth_session: AuthSession) -> impl IntoResponse {
  match auth_session.logout().await {
    Ok(_) => Redirect::to("/login").into_response(),
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  }
}

// pub async fn login_get(
//   Query(NextUrl { next }): Query<NextUrl>,
// ) -> LoginTemplate {
//   LoginTemplate { messages: messages.into_iter().collect(), next }
// }
