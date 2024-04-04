//! Authentication with axum-login.

mod password;
mod users;
mod web;

use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use db::{models::user::User, DbPool, Username};
use tower_sessions::service::SignedCookie;
use tower_sessions_sqlx_store::PostgresStore;

pub use self::{
  password::PasswordExt,
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
  /// Assert that the caller is logged in and not banned
  fn if_authenticated_get_user(&self, username: &Username) -> ApiResult<User>;
  /// Return whether the caller is logged in as username
  fn is_authenticated_and_not_banned(&self, username: &Username) -> bool;
  /// Return Ok(()) if the caller is authenticated as the given user.
  ///
  /// Return Err(ApiError::Forbidden) if caller is not authenticated.
  /// Return Err(ApiError::Unauthorized) if caller is authenticated but with non-matching username.
  fn caller_matches_payload(&self, username: &Username) -> ApiResult<()>;
}

impl AuthenticationExt for AuthSession {
  fn if_authenticated_get_user(&self, username: &Username) -> ApiResult<User> {
    let user =
      self.user.as_ref().map(|user| user.0.clone()).ok_or(ApiError::UnauthorizedPleaseLogin)?;

    if user.banned {
      return Err(ApiError::ForbiddenBanned);
    }

    if user.username == *username {
      Ok(user)
    } else {
      Err(ApiError::ForbiddenUsernameDoesNotMatchSession)
    }
  }

  fn is_authenticated_and_not_banned(&self, username: &Username) -> bool {
    self.user.as_ref().map(|user| user.0.username == *username).unwrap_or(false)
      && !self.user.as_ref().unwrap().0.banned
  }

  fn caller_matches_payload(&self, username: &Username) -> ApiResult<()> {
    self
      .user
      .as_ref()
      .map(|user| {
        if user.0.username == *username && !user.0.banned {
          Ok(())
        } else {
          Err(ApiError::UnauthorizedPleaseLogin)
        }
      })
      .unwrap_or_else(|| Err(ApiError::ForbiddenUsernameDoesNotMatchSession))
  }
}
