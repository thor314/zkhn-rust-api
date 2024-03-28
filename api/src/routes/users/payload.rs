use db::{models::user::User, About, Email, Password, Username};
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{error::ApiError, ApiResult};

/// Username, password, and optionally email, and about.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[schema(default = UserPayload::default, example=json!(UserPayload::default()))]
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

impl Default for UserPayload {
  fn default() -> Self {
    Self {
      username: Username("alice".to_string()),
      password: Password("password".to_string()),
      email:    None,
      about:    None,
    }
  }
}

impl TryFrom<UserPayload> for User {
  type Error = ApiError;

  /// Validate the payload and convert it into a User.
  fn try_from(value: UserPayload) -> Result<Self, Self::Error> {
    value.validate(&())?;
    let UserPayload { username, password, email, about } = value;
    let password_hash = password.hash()?;
    Ok(User::new(username, password_hash, email, about))
  }
}

impl UserPayload {
  /// Assume Comment Payload has already been validated.
  pub fn into_user(self) -> User {
    let password_hash = self.password.hash().unwrap();
    User::new(self.username, password_hash, self.email, self.about)
  }

  /// convenience method for testing
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
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[schema(default = UserUpdatePayload::default, example=json!(UserUpdatePayload::default()))]
pub struct UserUpdatePayload {
  #[garde(dive)]
  pub username: Username,
  #[garde(dive)]
  pub email:    Option<Email>,
  #[garde(dive)]
  pub about:    Option<About>,
}

impl Default for UserUpdatePayload {
  fn default() -> Self {
    Self {
      username: Username("alice".to_string()),
      email:    Some(Email("email@email.com".to_string())),
      about:    Some(About("about".to_string())),
    }
  }
}

impl UserUpdatePayload {
  /// convenience method for testing
  pub fn new(username: &str, email: Option<&str>, about: Option<&str>) -> ApiResult<Self> {
    let username = Username(username.to_string());
    if email.is_none() && about.is_none() {
      return Err(ApiError::MissingField("email or about must be provided".to_string()));
    }
    let email = email.map(|s| Email(s.to_string()));
    let about = about.map(|s| About(s.to_string()));
    let payload = Self { username, email, about };
    payload.validate(&())?;

    Ok(payload)
  }
}

/// Payload for `change_password`
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[schema(default = ChangePasswordPayload::default, example=json!(ChangePasswordPayload::default()))]
pub struct ChangePasswordPayload {
  #[garde(dive)]
  pub username:         Username,
  #[garde(dive)]
  pub current_password: Password,
  #[garde(dive)]
  pub new_password:     Password,
}

impl Default for ChangePasswordPayload {
  fn default() -> Self {
    Self {
      username:         Username("alice".to_string()),
      current_password: Password("password".to_string()),
      new_password:     Password("new_password".to_string()),
    }
  }
}

impl ChangePasswordPayload {
  /// convenience method for testing
  pub fn new(username: &str, current_password: &str, new_password: &str) -> ApiResult<Self> {
    let username = Username(username.to_string());
    let current_password = Password(current_password.to_string());
    let new_password = Password(new_password.to_string());
    let payload = Self { username, current_password, new_password };
    payload.validate(&())?;

    Ok(payload)
  }
}
