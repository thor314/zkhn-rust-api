// Axum-login cheatsheet there are 3 main traits to implement:
// - AuthUser - state the user struct, and provide methods to get the id and session_auth_hash
// - AuthnBackend - given a user_id, get the user from the database
//   - session_auth_hash, which is used to validate the session; provide some credentials to
//     validate the session
//   - get_user, which is used to load the user from the backend into the session.
// - AuthzBackend -
use anyhow::Context;
use axum::{async_trait, http::StatusCode, response::IntoResponse, Form};
use axum_login::{AuthUser, AuthnBackend, AuthzBackend, UserId};
// use db::models::comment::Comment;
use db::models::user::User;
use uuid::Uuid as Uid;

use crate::{error::MyError, DbPool, SharedState};

#[derive(Debug, Clone)]
// Newtype since cannot derive traits for types defined in other crates
pub struct UserNewType(User);

impl AuthUser for UserNewType {
  type Id = Uid;

  fn id(&self) -> Self::Id { self.0.id }

  // todo: this should probably be a session cookie or something, not the password
  fn session_auth_hash(&self) -> &[u8] { self.0.password_hash.as_bytes() }
}

#[derive(Debug, Clone)]
pub struct Credentials {
  id: Uid,
}

#[async_trait]
impl AuthnBackend for SharedState {
  type Credentials = Credentials;
  type Error = MyError;
  type User = UserNewType;

  async fn authenticate(
    &self,
    Credentials { id }: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    let user = self.get_user(&id).await?;
    // todo: authenticate with credentials
    Ok(user)
  }

  async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
    let user = db::get_user_from_id(&self.pool, *user_id).await.map(UserNewType);
    Ok(user)
  }
}

// a backend which can authorize users
impl AuthzBackend for SharedState {
  type Permission = Permissions;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Permissions {
  Default,
  LoggedIn,
  Mod,
}

type AuthSession = axum_login::AuthSession<SharedState>;
async fn login(mut auth_session: AuthSession, Form(creds): Form<Credentials>) -> impl IntoResponse {
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
