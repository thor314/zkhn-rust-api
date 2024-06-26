//! Authentication with axum-login.

mod password;
mod users;
mod web;

use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use db::{models::user::User, DbPool, Username};
use serde::{Deserialize, Serialize};
use tower_sessions::service::SignedCookie;
use tower_sessions_sqlx_store::PostgresStore;
use utoipa::ToSchema;

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
  /// Get the user from the session store if one exists
  fn get_user_from_session(&self) -> Option<User>;
  /// Get the user from the session store, or else return an Error
  ///
  /// Return Ok(user) if the caller is authenticated as the given user.
  /// Return Err(ApiError::Unauthorized) if caller is not logged in.
  /// Return Err(ApiError::Forbidden) if caller is banned.
  fn get_assert_user_from_session(&self) -> ApiResult<User>;
  /// Get the user from the session store, or else return an Error
  ///
  /// Return Ok(user) if the caller is authenticated as the given user.
  /// Return Err(ApiError::Unauthorized) if caller is not logged in.
  /// Return Err(ApiError::Forbidden) if caller is banned.
  fn get_assert_user_from_session_assert_match(&self, username: &Username) -> ApiResult<User>;
  /// Return whether the caller is logged in and not banned
  fn am_authenticated_and_not_banned(&self) -> bool;
}

impl AuthenticationExt for AuthSession {
  fn get_user_from_session(&self) -> Option<User> { self.user.clone().map(|u| u.0) }

  fn get_assert_user_from_session(&self) -> ApiResult<User> {
    let user = self.get_user_from_session().ok_or(ApiError::UnauthorizedPleaseLogin)?.clone();
    if user.banned {
      return Err(ApiError::ForbiddenBanned);
    }
    Ok(user)
  }

  fn get_assert_user_from_session_assert_match(&self, username: &Username) -> ApiResult<User> {
    let user = self.get_assert_user_from_session()?;
    if user.username != *username {
      return Err(ApiError::ForbiddenUsernameDoesNotMatchSession);
    }
    Ok(user)
  }

  fn am_authenticated_and_not_banned(&self) -> bool {
    self.user.as_ref().map(|user| !user.0.banned).unwrap_or(false)
  }
}
