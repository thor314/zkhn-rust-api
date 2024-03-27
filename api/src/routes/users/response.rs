use db::{models::user::User, AuthToken, Timestamp, Username};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
  // todo: success is redundant
  pub success: bool,
  pub username: Username,
  pub auth_token: AuthToken,
  pub auth_token_expiration_timestamp: Timestamp,
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
