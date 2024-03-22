use axum::Router;
use axum_login::{tower_sessions::SessionManagerLayer, AuthUser};
use db::{AuthToken, DbPool, PasswordHash, Username};
use serde::{Deserialize, Serialize};
use tower_sessions_sqlx_store::PostgresStore;
use uuid::Uuid;

/// A simplified User struct, to be used for authorization.
#[derive(Clone, Serialize, Deserialize)]
pub struct User {
  pub username:      Username,
  pub password_hash: Option<PasswordHash>,
  pub auth_token:    Option<AuthToken>,
}

// avoid logging sensitive info
impl std::fmt::Debug for User {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("User")
      .field("username", &self.username)
      .field("password", &"[redacted]")
      .field("access_token", &"[redacted]")
      .finish()
  }
}

impl AuthUser for User {
  type Id = Username;

  fn id(&self) -> Self::Id { self.username.clone() }

  fn session_auth_hash(&self) -> &[u8] {
    if let Some(access_token) = &self.auth_token {
      return access_token.0.as_bytes();
    } else if let Some(password) = &self.password_hash {
      return password.0.as_bytes();
    } else {
      tracing::error!("User has no password or access token");
      &[]
    }
  }
}

impl From<db::models::user::User> for User {
  fn from(user: db::models::user::User) -> Self {
    Self {
      username:      user.username,
      password_hash: Some(user.password_hash),
      auth_token:    user.auth_token,
    }
  }
}
