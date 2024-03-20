use db::models::user::User;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

// todo: sanitize and validate me here
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPayload {
  pub username: String,
  pub password: String,
  pub email:    Option<String>,
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
  pub fn new(username: &str, password: &str, email: Option<&str>, about: Option<&str>) -> Self {
    Self {
      username: username.to_string(),
      password: password.to_string(),
      email:    email.map(|s| s.to_string()),
      about:    about.map(|s| s.to_string()),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdatePayload {
  pub username: String,
  pub password: Option<String>,
  pub email:    Option<String>,
  pub about:    Option<String>,
}

impl UserUpdatePayload {
  pub fn new(
    username: &str,
    password: Option<&str>,
    email: Option<&str>,
    about: Option<&str>,
  ) -> Self {
    {
      Self {
        username: username.to_string(),
        password: password.map(|s| s.to_string()),
        email:    email.map(|s| s.to_string()),
        about:    about.map(|s| s.to_string()),
      }
    }
  }
}
