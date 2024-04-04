use db::{models::user::User, About, AuthToken, Email, PasswordHash, Timestamp, Username};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = CreateUserResponse::default, example=CreateUserResponse::default)]
pub struct CreateUserResponse {
  // hack(refactor): success is redundant
  pub success: bool,
  pub username: Username,
  pub auth_token: AuthToken,
  pub auth_token_expiration_timestamp: Timestamp,
}

impl CreateUserResponse {
  pub(crate) fn new(user: User, auth_token: AuthToken, auth_token_expiration: Timestamp) -> Self {
    Self {
      success: true,
      username: user.username,
      auth_token,
      auth_token_expiration_timestamp: auth_token_expiration,
    }
  }
}

impl From<User> for CreateUserResponse {
  fn from(user: User) -> Self {
    Self {
      success: true,
      username: user.username,
      auth_token: user.auth_token.unwrap_or_default(),
      auth_token_expiration_timestamp: user
        .auth_token_expiration
        .unwrap_or_else(Timestamp::default_expiration),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetUserResponse::default, example=GetUserResponse::default)]
pub struct GetUserResponse {
  pub username:          Username,
  pub created:           Timestamp,
  pub karma:             i32,
  pub about:             Option<About>,
  pub banned:            bool,
  // shadow_banned: removed
  /// private - authenticated access only, otherwise None
  pub email:             Option<Email>,
  /// private - authenticated access only, otherwise None
  pub show_dead:         Option<bool>,
  pub show_private_user_data: bool,
}

impl GetUserResponse {
  pub fn new(user: User, is_authenticated: bool) -> Self {
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
    }
  }
}
#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = AuthenticateUserResponse::default, example=AuthenticateUserResponse::default)]
pub struct AuthenticateUserResponse {
  // hack(redundant)
  pub success:        bool,
  pub username:       Username,
  pub banned:         bool,
  pub karma:          i32,
  pub contains_email: bool,
  pub show_dead:      bool,
  pub is_moderator:   bool,
  // shadow banned removed
}

impl AuthenticateUserResponse {
  pub fn new(user: User) -> Self {
    Self {
      success:        true,
      username:       user.username,
      banned:         user.banned,
      karma:          user.karma,
      contains_email: user.email.is_some(),
      show_dead:      user.show_dead,
      is_moderator:   user.is_moderator,
    }
  }
}
