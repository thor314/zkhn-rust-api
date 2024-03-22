use db::{Password, Username};
use oauth2::CsrfToken;
use serde::Deserialize;

/// Users may log in either via password or via OAuth
#[derive(Debug, Clone, Deserialize)]
pub enum Credentials {
  Password(PasswordCreds),
  OAuth(OAuthCreds),
}

/// Credentials for logging in with a username and password.
#[derive(Debug, Clone, Deserialize)]
pub struct PasswordCreds {
  pub username: Username,
  pub password: Password,
  pub next:     Option<String>,
}

/// Credentials for logging in with an OAuth code.
#[derive(Debug, Clone, Deserialize)]
pub struct OAuthCreds {
  pub code:      String,
  pub old_state: CsrfToken,
  pub new_state: CsrfToken,
}
