use db::models::user::User;
use serde::{Deserialize, Serialize};

use crate::error::{ApiError, PayloadError};

// todo: sanitize and validate me here
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPayload {
  pub username: String,
  pub password: String,
  pub email:    String,
  pub about:    Option<String>,
}

impl TryFrom<UserPayload> for User {
  type Error = PayloadError;

  fn try_from(value: UserPayload) -> Result<Self, Self::Error> {
    let UserPayload { username, password, email, about } = value;
    // todo: sanitize and validate me
    Ok(User::new(username, password, email, about))
  }
}
