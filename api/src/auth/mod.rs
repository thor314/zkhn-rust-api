//! Authentication with axum-login.
//!
//! Usage in a handler:
//! ```rust
//! use axum::{http::StatusCode, response::IntoResponse};
//! pub async fn protected(auth_session: api::AuthSession) -> impl IntoResponse {
//!   match auth_session.user {
//!     Some(user) => StatusCode::OK, // do stuff
//!     None => StatusCode::INTERNAL_SERVER_ERROR,
//!   }
//! }
//! ```
//!
//! For more advanced setting of permissions, see:
//! https://github.com/maxcountryman/axum-login/blob/main/examples/permissions/src/users.rs#L107
//!
//! Axum-login cheatsheet
//! ---------------------
//!
//! there are 2 main traits to implement:
//! - AuthUser - Implement for a user that can be authenticated and authorized. Requires methods for
//!   getting the user's unique id and the password hash or access token.
//! - AuthnBackend - given a user_id, get the user from the database
//!   - session_auth_hash, which is used to validate the session; provide some credentials to
//!     validate the session
//!   - get_user, which is used to load the user from the backend into the session.

use axum_login::AuthUser;
use db::Username;
use serde::Deserialize;

// mod jank;
// mod oauth;
mod auth_handlers;
mod auth_user;
mod backend;
mod credentials;
mod password;

pub use self::backend::Backend;

pub type AuthSession = axum_login::AuthSession<Backend>;

/// This allows us to extract the "next" field from the query string. We use this
/// to redirect after log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
  next: Option<String>,
}

/// todo: what does this do?
#[derive(Debug, Deserialize)]
struct UserInfo {
  login: String,
}

#[cfg(test)]
mod test {
  use axum::{
    body::Body,
    http::{self, Request, StatusCode},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
  };
  use axum_login::{login_required, AuthManagerLayerBuilder};
  use db::{models::user::User, DbPool};
  use http_body_util::BodyExt;
  use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
  use serde::Serialize;
  use serde_json::json;
  use sqlx::PgPool;
  use tower::ServiceExt;

  use self::credentials::Credentials;
  use super::{auth_handlers::login_handler, credentials::password_creds::PasswordCreds, *};
  use crate::{sessions::get_session_layer, SharedState};

  async fn get_server(pool: &DbPool) -> Router {
    let state = SharedState::new(pool.clone());
    let session_layer = get_session_layer(pool).await.unwrap();
    let backend = Backend::new_with_default_client(pool.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    Router::new()
    // login protected routes go above the login_required layer
    .route("/logout", post(auth_handlers::logout_handler))
    .route_layer(login_required!(Backend, login_url = "/login"))
    // unprotected routes (like "/login") go here
    .route("/login", post(login_handler))
    .layer(auth_layer)
  }

  #[sqlx::test(migrations = "../db/migrations")]
  async fn test_auth_password(pool: PgPool) {
    let app = get_server(&pool).await;

    let creds = Credentials::Password(PasswordCreds::new("alice", "password", None));
    // let creds = PasswordCreds::new("alice", "password", None);
    let json_body = json!(&creds).to_string();

    let login_request = Request::builder()
      .uri("/login")
      .method("POST")
      .header(http::header::CONTENT_TYPE, "application/json")
      .body(json_body)
      .unwrap();
    let response = app.clone().oneshot(login_request).await.unwrap();
    // assert_eq!(response.status(), StatusCode::OK);
    let body = &response.into_body().collect().await.unwrap();
    dbg!(&body);
    panic!();

    let cookies = response
      .headers()
      .get_all("Set-Cookie")
      .iter()
      .map(|c| c.to_str().unwrap().to_owned())
      .collect::<Vec<_>>();
    assert!(!cookies.is_empty(), "No session cookie found in response");


    // let logout_request =
    //   Request::builder().uri("/logout").method("POST").body(Body::empty()).unwrap();
    // let response = app.clone().oneshot(logout_request).await.unwrap();
    // dbg!(response);

    panic!()
  }
}
