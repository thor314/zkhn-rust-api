// Axum-login cheatsheet there are 3 main traits to implement:
// - AuthUser - state the user struct, and provide methods to get the id and session_auth_hash
// - AuthnBackend - given a user_id, get the user from the database
//   - session_auth_hash, which is used to validate the session; provide some credentials to
//     validate the session
//   - get_user, which is used to load the user from the backend into the session.
// - AuthzBackend -
use anyhow::Context;
use axum::{async_trait, http::StatusCode, response::IntoResponse, Form};
use axum_login::{tower_sessions::{session, SessionStore}, AuthUser, AuthnBackend, AuthzBackend, UserId};
// use db::models::comment::Comment;
use db::models::user::User;
use tokio::task;
use uuid::Uuid as Uid;

use crate::{error::ApiError, DbPool, SharedState};
// use crate::{error::ApiError, session::get_session_manager_layer, DbPool, SharedState};

/// Axum extractor for the current user session
pub type AuthSession = axum_login::AuthSession<Backend>;

// todo verify
pub async fn login(
  mut auth_session: AuthSession,
  Form(creds): Form<Credentials>,
) -> impl IntoResponse {
  let user = match auth_session.authenticate(creds.clone()).await {
    Ok(Some(user)) => user,
    Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
    Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
  };

  if auth_session.login(&user).await.is_err() {
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
  }

  axum::response::Redirect::to("/").into_response()
}

// pub async fn get_auth_manager_layer(
//   pool: DbPool,
// ) -> axum_login::AuthManagerLayer<Backend, impl SessionStore> {
//   let backend = Backend::new(pool.clone());
//   let session_layer = get_session_manager_layer(&pool).await;
//   let auth_manager_layer = axum_login::AuthManagerLayerBuilder::new(backend, session_layer).build();
//   auth_manager_layer
// }

#[derive(Debug, Clone)]
// Newtype since cannot derive traits for types defined in other crates
pub struct UserAuthWrapper(User);

impl AuthUser for UserAuthWrapper {
  type Id = Uid;

  fn id(&self) -> Self::Id { self.0.id }

  // todo: this should probably be a session cookie or something, not the password
  fn session_auth_hash(&self) -> &[u8] { self.0.password_hash.as_bytes() }
}

#[derive(Debug, Clone)]
pub struct Credentials {
  id:           Uid,
  pub username: String,
  pub password: String,
  // todo: should this live here?
  // /// where to redirect the user after login
  // pub redirect_url: Option<String>,
}

#[derive(Clone)]
pub struct Backend {
  pool: DbPool,
}

impl Backend {
  pub fn new(pool: DbPool) -> Self { Self { pool } }
}

#[async_trait]
impl AuthnBackend for Backend {
  type Credentials = Credentials;
  type Error = ApiError;
  type User = UserAuthWrapper;

  async fn authenticate(
    &self,
    credentials: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    let user = self.get_user(&credentials.id).await?;
    // Verifying the password is blocking and potentially slow, so use `spawn_blocking`.
    task::spawn_blocking(move || {
      Ok(user.filter(|user| user.0.verify_password(&credentials.password).is_ok()))
    })
    .await?
  }

  async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
    let user = db::get_user_from_id(&self.pool, *user_id).await.map(UserAuthWrapper);
    Ok(user)
  }
}

// todo: authz?
// a backend which can authorize users
// impl AuthzBackend for Backend {
//   type Permission = Permissions;
// }

// #[derive(Debug, Clone, Eq, PartialEq, Hash)]
// pub enum Permissions {
//   Default,
//   LoggedIn,
//   Mod,
// }

mod protected {
  use super::*;

  pub async fn protected(auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.user {
      Some(user) => StatusCode::OK,
      // do something protected
      None => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}
