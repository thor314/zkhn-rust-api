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

use self::backend::Backend;

// mod jank;
// mod oauth;
mod auth_user;
mod backend;
mod credentials;
mod password;

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

#[derive(Debug, Deserialize, Clone)]
pub struct User(pub db::models::user::User);

impl AuthUser for User {
  type Id = Username;

  fn id(&self) -> Self::Id { self.0.username.clone() }

  // todo: jank
  fn session_auth_hash(&self) -> &[u8] {
    if let Some(access_token) = &self.0.auth_token {
      access_token.0.as_bytes()
    } else {
      tracing::error!("User has no password or access token");
      &[]
    }
  }
}
