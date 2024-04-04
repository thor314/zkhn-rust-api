use db::{models::user::User, About, Email, Password, Username};
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{auth::PasswordExt, error::ApiError, ApiResult};

/// Username, password, and optionally email, and about.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(default = UserPayload::default, example=UserPayload::default)]
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

impl UserPayload {
  pub async fn into_user(self) -> User {
    let password_hash = self.password.hash().await;
    User::new(self.username, password_hash, self.email, self.about)
  }

  /// convenience method for testing
  pub fn new(
    username: &str,
    password: &str,
    email: Option<&str>,
    about: Option<&str>,
  ) -> ApiResult<Self> {
    let username = username.into();
    let password = password.into();
    let email = email.map(|s| s.into());
    let about = about.map(|s| s.into());
    let payload = Self { username, password, email, about };
    payload.validate(&())?;
    Ok(payload)
  }
}

/// Update user details.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(default = UserUpdatePayload::default, example=UserUpdatePayload::default)]
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
      username: Username::default(),
      email:    Some("email@email.com".into()),
      about:    Some("about".into()),
    }
  }
}

impl UserUpdatePayload {
  /// convenience method for testing
  pub fn new(username: &str, email: Option<&str>, about: Option<&str>) -> ApiResult<Self> {
    let username = username.into();
    if email.is_none() && about.is_none() {
      return Err(ApiError::BadRequest("email or about must be provided".to_string()));
    }
    let email = email.map(|s| s.into());
    let about = about.map(|s| s.into());
    let payload = Self { username, email, about };
    payload.validate(&())?;

    Ok(payload)
  }
}

/// Payload for `change_password`
#[derive(Debug, Default, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(default = ChangePasswordPayload::default, example=ChangePasswordPayload::default)]
pub struct ChangePasswordPayload {
  #[garde(dive)]
  pub username:         Username,
  #[garde(dive)]
  pub current_password: Password,
  #[garde(dive)]
  pub new_password:     Password,
}

impl ChangePasswordPayload {
  /// convenience method for testing
  pub fn new(username: &str, current_password: &str, new_password: &str) -> ApiResult<Self> {
    let username = username.into();
    let current_password = current_password.into();
    let new_password = new_password.into();
    let payload = Self { username, current_password, new_password };
    payload.validate(&())?;

    Ok(payload)
  }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
#[schema(example = CredentialsPayload::default, default = CredentialsPayload::default)]
pub struct CredentialsPayload {
  #[garde(dive)]
  pub username: Username,
  #[garde(dive)]
  pub password: Password,
  #[garde(skip)]
  pub next:     Option<String>,
}

impl CredentialsPayload {
  pub fn new(username: &str, password: &str, next: Option<String>) -> Self {
    Self { username: username.into(), password: password.into(), next }
  }
}
