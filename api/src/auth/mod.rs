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
  /// Get the user from the session store, if one exists
  ///
  /// Return Ok(user) if the caller is authenticated as the given user.
  /// Return Err(ApiError::Forbidden) if caller is not authenticated.
  fn get_user_from_session(&self) -> ApiResult<User>;
  /// Assert that the caller is logged in and not banned
  ///
  /// Return Ok(user) if the caller is authenticated as the given user.
  /// Return Err(ApiError::Forbidden) if caller is not authenticated.
  /// Return Err(ApiError::Unauthorized) if caller is authenticated but with non-matching username.
  fn if_authenticated_get_user(&self, username: &Username) -> ApiResult<User>;
  /// Return whether the caller is logged in as username
  fn is_authenticated_and_not_banned(&self, username: &Username) -> bool;
}

impl AuthenticationExt for AuthSession {
  fn get_user_from_session(&self) -> ApiResult<User> {
    let user = self.user.as_ref().ok_or(ApiError::UnauthorizedPleaseLogin)?.clone().0;
    if user.banned {
      return Err(ApiError::ForbiddenBanned);
    }
    Ok(user)
  }

  fn if_authenticated_get_user(&self, username: &Username) -> ApiResult<User> {
    let user = self.get_user_from_session()?;
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
}
