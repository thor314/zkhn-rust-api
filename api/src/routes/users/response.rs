use db::{models::user::User, AuthToken, Timestamp, Username};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(default = UserResponse::default, example=UserResponse::default)]
pub struct UserResponse {
  // todo(refactor): success is redundant
  pub success: bool,
  pub username: Username,
  pub auth_token: AuthToken,
  pub auth_token_expiration_timestamp: Timestamp,
}

impl Default for UserResponse {
  fn default() -> Self {
    Self {
      success: false,
      username: Username("alice".to_string()),
      auth_token: AuthToken("auth_token".to_string()),
      auth_token_expiration_timestamp: Timestamp(chrono::Utc::now()),
    }
  }
}

impl UserResponse {
  pub(crate) fn new(user: User, auth_token: AuthToken, auth_token_expiration: Timestamp) -> Self {
    Self {
      success: true,
      username: user.username,
      auth_token,
      auth_token_expiration_timestamp: auth_token_expiration,
    }
  }
}

impl From<User> for UserResponse {
  fn from(user: User) -> Self {
    Self {
      success: true,
      username: user.username,
      auth_token: user.auth_token.unwrap(),
      auth_token_expiration_timestamp: user.auth_token_expiration.unwrap(),
    }
  }
}
