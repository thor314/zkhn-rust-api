use super::*;

/// Username, password, and optionally email, and about.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(default = CreateUserPayload::default, example=CreateUserPayload::default)]
pub struct CreateUserPayload {
  #[garde(dive)]
  pub username: Username,
  #[garde(dive)]
  pub password: Password,
  #[garde(dive)]
  pub email:    Option<Email>,
  #[garde(dive)]
  pub about:    Option<About>,
}

impl CreateUserPayload {
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

  pub fn bob() -> Self {
    Self::new("bob", "password", Some("bob@email.com"), Some("about bob")).unwrap()
  }
}

/// Update user details.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(default = UserUpdatePayload::default, example=UserUpdatePayload::default)]
pub struct UserUpdatePayload {
  #[garde(dive)]
  pub email:     Option<Email>,
  #[garde(dive)]
  pub about:     Option<About>,
  #[garde(skip)]
  pub show_dead: Option<bool>,
}

impl Default for UserUpdatePayload {
  fn default() -> Self {
    Self {
      email:     Some("email@email.com".into()),
      about:     Some("about".into()),
      show_dead: Some(false),
    }
  }
}

impl UserUpdatePayload {
  /// convenience method for testing
  pub fn new(email: Option<&str>, about: Option<&str>, show_dead: Option<bool>) -> ApiResult<Self> {
    if email.is_none() && about.is_none() {
      return Err(ApiError::BadRequest("email or about must be provided".to_string()));
    }
    let email = email.map(|s| s.into());
    let about = about.map(|s| s.into());
    let payload = Self { email, about, show_dead };
    payload.validate(&())?;

    Ok(payload)
  }
}

/// Payload for `change_password`
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(default = ChangePasswordPayload::default, example=ChangePasswordPayload::default)]
pub struct ChangePasswordPayload {
  #[garde(dive)]
  pub username:             Username,
  #[garde(dive)]
  pub current_password:     Option<Password>,
  #[garde(dive)]
  pub reset_password_token: Option<ResetPasswordToken>,
  #[garde(dive)]
  pub new_password:         Password,
}

impl Default for ChangePasswordPayload {
  fn default() -> Self {
    Self {
      username:             "alice".into(),
      current_password:     Some("password".into()),
      reset_password_token: None,
      new_password:         "new_password".into(),
    }
  }
}

impl ChangePasswordPayload {
  /// convenience method for testing
  pub fn new(
    username: &str,
    current_password: Option<&str>,
    reset_password_token: Option<&str>,
    new_password: &str,
  ) -> ApiResult<Self> {
    let username = username.into();
    let current_password = current_password.map(Password::from);
    let reset_password_token = reset_password_token.map(ResetPasswordToken::from);
    let new_password = new_password.into();
    let payload = Self { username, current_password, reset_password_token, new_password };
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

  pub fn bob() -> Self { Self::new("bob", "password", None) }
}
