//! A simplified auth struct for testing auth
//! ref: https://github.com/maxcountryman/axum-login/blob/main/examples/multi-auth/src/users.rs
use axum_login::AuthUser;
use db::Username;
use serde::{Deserialize, Serialize};

/// Wrapper for the db user model that implements AuthUser.
// ref: https://github.com/maxcountryman/axum-login/blob/main/examples/multi-auth/src/users.rs#L16
#[derive(Debug, Deserialize, Clone)]
pub struct User(pub db::models::user::User);

impl AuthUser for User {
  type Id = Username;

  fn id(&self) -> Self::Id { self.0.username.clone() }

  // ref: https://github.com/maxcountryman/axum-login/blob/main/examples/multi-auth/src/users.rs#L43
  fn session_auth_hash(&self) -> &[u8] {
    if let Some(access_token) = &self.0.auth_token {
      access_token.0.as_bytes()
    } else {
      tracing::warn!("User has no access token, defaulting to password hash");
      self.0.password_hash.0.as_bytes()
    }
  }
}
