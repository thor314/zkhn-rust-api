use axum_login::{AuthUser, AuthnBackend, UserId};
use db::{models::user::User, DbPool, Username};
use serde::Serialize;
use tokio::task;

use super::PasswordExt;
use crate::{error::ApiError, CredentialsPayload};

#[derive(Debug, Clone, Serialize)]
pub struct UserWrapper(pub User);

impl AuthUser for UserWrapper {
  type Id = Username;

  fn id(&self) -> Self::Id { self.0.username.clone() }

  // We use the password hash as the auth hash
  // this means when the user changes their password, the auth session becomes invalid.
  fn session_auth_hash(&self) -> &[u8] { self.0.password_hash.0.as_bytes() }
}

#[derive(Debug, Clone)]
pub struct AuthBackend {
  db: DbPool,
}

impl AuthBackend {
  pub fn new(db: DbPool) -> Self { Self { db } }
}

#[axum::async_trait]
impl AuthnBackend for AuthBackend {
  type Credentials = CredentialsPayload;
  type Error = ApiError;
  type User = UserWrapper;

  /// Authenticate a user.
  ///
  /// Ok(Some(User)) - If the user exists, and the password is correct
  /// Ok(None) - Never
  /// Err(ApiError) - If the user doesn't exist, or the password is incorrect.
  async fn authenticate(
    &self,
    creds: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    let user: Option<Self::User> = self.get_user(&creds.username).await?;

    if let Some(user) = user {
      if creds.password.hash_and_verify(&user.0.password_hash).await.is_err() {
        tracing::error!("Incorrect password: {:?}", creds);
        Err(ApiError::Unauthorized("Incorrect password".to_string())) // wrong password
      } else {
        Ok(Some(user)) // right password
      }
    } else {
      Err(ApiError::DbEntryNotFound("Couldn't find user".to_string()))
    }
  }

  async fn get_user(&self, username: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
    let user = db::queries::users::get_user(&self.db, username).await?;
    Ok(Some(UserWrapper(user)))
  }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<AuthBackend>;
