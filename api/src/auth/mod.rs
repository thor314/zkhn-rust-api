//! Authentication with axum-login.

mod users;
mod web;

use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use db::{DbPool, Username};
use tower_sessions::service::SignedCookie;
use tower_sessions_sqlx_store::PostgresStore;

pub use self::{
  users::{AuthBackend, AuthSession},
  web::{login_post_internal, logout_post_internal},
};
use crate::{sessions::MySessionManagerLayer, ApiError, ApiResult};

pub type MyAuthLayer = AuthManagerLayer<AuthBackend, PostgresStore, SignedCookie>;

pub fn get_auth_layer(pool: DbPool, session_layer: MySessionManagerLayer) -> MyAuthLayer {
  let backend = AuthBackend::new(pool);
  AuthManagerLayerBuilder::new(backend, session_layer).build()
}

pub(crate) trait AuthenticationExt {
  /// Assert that the caller is logged in.
  fn assert_authenticated(&self) -> ApiResult<()>;
  /// Is the caller logged in?
  fn is_authenticated(&self) -> bool;
  /// Return Ok(()) if the caller is authenticated as the given user.
  ///
  /// Return Err(ApiError::Forbidden) if the caller is not authenticated.
  fn caller_matches_payload(&self, username: &Username) -> ApiResult<()>;
}

impl AuthenticationExt for AuthSession {
  fn assert_authenticated(&self) -> ApiResult<()> {
    if self.user.is_none() {
      return Err(crate::ApiError::Forbidden("login required".to_string()));
    }

    Ok(())
  }

  fn is_authenticated(&self) -> bool { self.user.is_some() }

  fn caller_matches_payload(&self, username: &Username) -> ApiResult<()> {
    self
      .user
      .as_ref()
      .map(|user| {
        if user.0.username == *username {
          Ok(())
        } else {
          Err(ApiError::Unauthorized(format!("Caller must equal user: {username}")))
        }
      })
      .unwrap_or_else(|| Err(ApiError::Forbidden("login required".to_string())))
  }
}
