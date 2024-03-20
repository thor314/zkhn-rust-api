use db::models::user::User;
use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::{error::ApiError, ApiResult};

// todo: sanitize and validate me here
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserPayload {
  #[garde(dive)]
  pub username: Username,
  #[garde(dive)]
  pub password: Password,
  #[garde(dive)]
  pub email:    Option<Email>,
  #[garde(dive)]
  pub about:    Option<About>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[garde(transparent)]
pub struct About(#[garde(ascii, length(min = 0, max = 400))] pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[garde(transparent)]
pub struct Password(#[garde(ascii, length(min = 8, max = 25))] pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[garde(transparent)]
pub struct Username(#[garde(ascii, length(min = 3, max = 25))] pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[garde(transparent)]
pub struct Email(#[garde(email)] pub String);

impl std::fmt::Display for Email {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}

impl TryFrom<UserPayload> for User {
  type Error = ApiError;

  fn try_from(value: UserPayload) -> Result<Self, Self::Error> {
    let UserPayload { username, password, email, about } = value;
    Ok(User::new(username.0, password.0, email.map(|s| s.0), about.map(|s| s.0)))
  }
}

impl UserPayload {
  pub fn new(
    username: &str,
    password: &str,
    email: Option<&str>,
    about: Option<&str>,
  ) -> ApiResult<Self> {
    let username = Username(username.to_string());
    username.validate(&())?;
    let password = Password(password.to_string());
    password.validate(&())?;
    let email = email.map(|s| Email(s.to_string()));
    email.validate(&())?;
    let about = about.map(|s| About(s.to_string()));
    about.validate(&())?;

    Ok(Self { username, password, email, about })
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
