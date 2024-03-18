use db::models::user::User;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

// todo: sanitize and validate me here
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPayload {
  pub username: String,
  pub password: String,
  pub email:    String,
  pub about:    Option<String>,
}

impl TryFrom<UserPayload> for User {
  type Error = ApiError;

  fn try_from(value: UserPayload) -> Result<Self, Self::Error> {
    let UserPayload { username, password, email, about } = value;
    // todo: sanitize and validate me
    Ok(User::new(username, password, email, about))
  }
}

impl UserPayload {
  pub fn new(username: &str, password: &str, email: &str, about: Option<&str>) -> Self {
    {
      Self {
        username: username.to_string(),
        password: password.to_string(),
        email:    email.to_string(),
        about:    about.map(|s| s.to_string()),
      }
    }
  }
}
