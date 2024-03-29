// use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use db::{
  models::user::User, password::verify_password, queries::get_user, DbPool, Password, Username,
};
// use password_auth::verify_password;
use serde::{Deserialize, Serialize};
// use sqlx::{FromRow, PgPool};
use tokio::task;
use utoipa::ToSchema;

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

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
  // #[error(transparent)]
  // Sqlx(#[from] sqlx::Error),
  #[error(transparent)]
  TaskJoin(#[from] task::JoinError),
}

#[axum::async_trait]
impl AuthnBackend for AuthBackend {
  type Credentials = CredentialsPayload;
  type Error = ApiError;
  type User = UserWrapper;

  async fn authenticate(
    &self,
    creds: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    let user: Option<Self::User> = self.get_user(&creds.username).await?;

    // Verifying the password is blocking and potentially slow, so we'll do so via
    // `spawn_blocking`.
    task::spawn_blocking(|| {
      // We're using password-based authentication--this works by comparing our form
      // input with an argon2 password hash.
      dbg!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA", &user);
      let user = user.filter(|user| verify_password(&user.0.password_hash, creds.password).is_ok());
      dbg!("BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB", &user);
      Ok(user)
      // Ok(user.filter(|user| verify_password(&user.0.password_hash, creds.password).is_ok()));
    })
    .await?
  }

  async fn get_user(&self, username: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
    let user = db::queries::users::get_user(&self.db, username).await?.map(UserWrapper);

    Ok(user)
  }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<AuthBackend>;
