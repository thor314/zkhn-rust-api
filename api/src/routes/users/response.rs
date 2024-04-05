use db::{models::user::User, About, AuthToken, Email, PasswordHash, Timestamp, Username};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::MINIMUM_KARMA_TO_DOWNVOTE;

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
#[schema(default = CreateUserResponse::default, example=CreateUserResponse::default)]
pub struct CreateUserResponse {
  pub username: Username,
  pub auth_token: AuthToken,
  pub auth_token_expiration_timestamp: Timestamp,
}

impl CreateUserResponse {
  pub(crate) fn new(user: User, auth_token: AuthToken, auth_token_expiration: Timestamp) -> Self {
    Self {
      username: user.username,
      auth_token,
      auth_token_expiration_timestamp: auth_token_expiration,
    }
  }
}

impl From<User> for CreateUserResponse {
  fn from(user: User) -> Self {
    Self {
      username: user.username,
      auth_token: user.auth_token.unwrap_or_default(),
      auth_token_expiration_timestamp: user
        .auth_token_expiration
        .unwrap_or_else(Timestamp::default_expiration),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
#[schema(default = GetUserResponse::default, example=GetUserResponse::default)]
pub struct GetUserResponse {
  pub username:               Username,
  pub created:                Timestamp,
  pub karma:                  i32,
  pub about:                  Option<About>,
  pub banned:                 bool,
  // shadow_banned: removed
  /// private - authenticated access only, otherwise None
  pub email:                  Option<Email>,
  /// private - authenticated access only, otherwise None
  pub show_dead:              Option<bool>,
  pub show_private_user_data: bool,
  pub auth_user:              AuthUserResponseInternal,
}

impl GetUserResponse {
  pub fn new(user: User, is_authenticated: bool) -> Self {
    let auth_user = AuthUserResponseInternal::new(user.clone(), is_authenticated);
    let email = user.email.filter(|_| is_authenticated);
    let show_dead = Some(user.show_dead).filter(|_| is_authenticated);
    Self {
      username: user.username,
      created: user.created,
      karma: user.karma,
      about: user.about,
      banned: user.banned,
      email,
      show_dead,
      show_private_user_data: is_authenticated,
      auth_user,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
#[schema(default = AuthenticateUserResponse::default, example=AuthenticateUserResponse::default)]
pub struct AuthenticateUserResponse {
  pub username:       Username,
  pub banned:         bool,
  pub karma:          i32,
  pub contains_email: bool,
  pub show_dead:      bool,
  pub is_moderator:   bool,
  // shadow banned removed
  auth_user:          AuthUserResponseInternal,
}

impl AuthenticateUserResponse {
  pub fn new(user: User, is_authenticated: bool) -> Self {
    let auth_user = AuthUserResponseInternal::new(user.clone(), is_authenticated);
    Self {
      username: user.username,
      banned: user.banned,
      karma: user.karma,
      contains_email: user.email.is_some(),
      show_dead: user.show_dead,
      is_moderator: user.is_moderator,
      auth_user,
    }
  }
}

/// Local authentication state, mirroring middleware on reference implementation
///
/// ref: https://github.com/thor314/zkhn/blob/main/rest-api/middlewares/index.js#L36
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
#[schema(default = AuthUserResponseInternal::default, example=AuthUserResponseInternal::default)]
#[serde(rename_all = "camelCase")]
pub struct AuthUserResponseInternal {
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

impl AuthUserResponseInternal {
  /// Create a new AuthLocal from a User
  pub fn new(user: User, is_authenticated: bool) -> Self {
    if is_authenticated {
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
    } else {
      Self::default()
    }
  }

  /// Create a new AuthLocal without authentication
  pub fn new_unauthenticated(banned: bool) -> Self { Self { banned, ..Default::default() } }
}
