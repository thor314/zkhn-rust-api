use db::{models::user::User, About, Email, Username};
use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::{error::ApiError, ApiResult};

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

impl TryFrom<UserPayload> for User {
  type Error = ApiError;

  fn try_from(value: UserPayload) -> Result<Self, Self::Error> {
    value.validate(&())?;
    let UserPayload { username, password, email, about } = value;
    Ok(User::new(username.0, password.0, email, about))
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
    let password = Password(password.to_string());
    let email = email.map(|s| Email(s.to_string()));
    let about = about.map(|s| About(s.to_string()));
    let payload = Self { username, password, email, about };
    payload.validate(&())?;
    Ok(payload)
  }
}

/// Update user details.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserUpdatePayload {
  #[garde(dive)]
  pub username: Username,
  #[garde(dive)]
  pub password: Option<Password>,
  #[garde(dive)]
  pub email:    Option<Email>,
  #[garde(dive)]
  pub about:    Option<About>,
}

impl UserUpdatePayload {
  pub fn new(
    username: &str,
    password: Option<&str>,
    email: Option<&str>,
    about: Option<&str>,
  ) -> ApiResult<Self> {
    let username = Username(username.to_string());
    let password= password.map(|s| Password(s.to_string()));
    let email = email.map(|s| Email(s.to_string()));
    let about = about.map(|s| About(s.to_string()));
    let payload = Self {
        username,
        password,
        email,
        about,
    };
    payload.validate(&())?;

    Ok(payload)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[garde(transparent)]
pub struct Password(#[garde(ascii, length(min = 8, max = 25))] pub String);
