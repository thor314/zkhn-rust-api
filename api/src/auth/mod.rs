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
use crate::{sessions::MySessionManagerLayer, ApiError, ApiResult, MINIMUM_KARMA_TO_DOWNVOTE};

pub type MyAuthLayer = AuthManagerLayer<AuthBackend, PostgresStore, SignedCookie>;

pub fn get_auth_layer(pool: DbPool, session_layer: MySessionManagerLayer) -> MyAuthLayer {
  let backend = AuthBackend::new(pool);
  AuthManagerLayerBuilder::new(backend, session_layer).build()
}

pub(crate) trait AuthenticationExt {
  /// Get the user from the session store, or else return an Error
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
  /// Return whether the caller is logged in
  fn is_authenticated_and_not_banned(&self) -> bool;
  /// Return whether the caller is logged in as username
  fn is_username_authenticated_and_not_banned(&self, username: &Username) -> bool;
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

  fn is_authenticated_and_not_banned(&self) -> bool {
    self.user.as_ref().map(|user| !user.0.banned).unwrap_or(false)
  }

  fn is_username_authenticated_and_not_banned(&self, username: &Username) -> bool {
    self.is_authenticated_and_not_banned() && self.user.as_ref().unwrap().0.username == *username
  }
}

/// Local authentication state, mirroring middleware on reference implementation
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/middlewares/index.js#L36
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
#[schema(default = AuthLocal::default, example=AuthLocal::default)]
#[serde(rename_all = "camelCase")]
pub struct AuthLocal {
  pub user_signed_in:   bool,
  pub username:         Option<Username>,
  pub karma:            Option<i32>,
  pub contains_email:   Option<bool>,
  pub show_dead:        bool,
  pub show_downvote:    bool,
  pub is_moderator:     Option<bool>,
  // shadow_banned: removed
  pub banned:           bool,
  pub cookies_included: bool,
}

impl AuthLocal {
  /// Create a new AuthLocal from a User
  pub fn new(user: User) -> Self {
    Self {
      user_signed_in:   true,
      username:         Some(user.username.clone()),
      karma:            Some(user.karma),
      contains_email:   Some(user.email.is_some()),
      show_dead:        user.show_dead,
      show_downvote:    user.karma >= MINIMUM_KARMA_TO_DOWNVOTE,
      is_moderator:     Some(user.is_moderator),
      banned:           user.banned,
      cookies_included: true,
    }
  }

  /// Create a new AuthLocal without authentication
  pub fn new_unauthenticated(banned: bool) -> Self { Self { banned, ..Default::default() } }
}
