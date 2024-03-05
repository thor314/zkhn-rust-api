//! Authentication with axum-login.
//!
//! Usage in a handler:
//! ```rust
//! use axum::{http::StatusCode, response::IntoResponse};
//! pub async fn protected(auth_session: api::AuthSession) -> impl IntoResponse {
//!   match auth_session.user {
//!     Some(user) => StatusCode::OK, // do stuff
//!     None => StatusCode::INTERNAL_SERVER_ERROR,
//!   }
//! }
//! ```
//!
//! For more advanced setting of permissions, see:
//! https://github.com/maxcountryman/axum-login/blob/main/examples/permissions/src/users.rs#L107

// Axum-login cheatsheet
// ---------------------
//
// there are 3 main traits to implement:
// - AuthUser - state the user struct, and provide methods to get the id and session_auth_hash
// - AuthnBackend - given a user_id, get the user from the database
//   - session_auth_hash, which is used to validate the session; provide some credentials to
//     validate the session
//   - get_user, which is used to load the user from the backend into the session.
// - AuthzBackend -
use anyhow::Context;
use axum::{
  async_trait,
  http::StatusCode,
  response::{IntoResponse, Redirect},
  routing::post,
  Form, Router,
};
use axum_login::{
  login_required,
  tower_sessions::{session, SessionStore},
  AuthManagerLayerBuilder, AuthUser, AuthnBackend, AuthzBackend, UserId,
};
use db::models::user::User;
use serde::{Deserialize, Serialize};
use tokio::task;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
use uuid::Uuid;

use crate::{error::{ApiError, RouteError}, ApiResult, DbPool, SharedState};

/// Axum extractor for the current user session
pub type AuthSession = axum_login::AuthSession<Backend>;

/// construct the auth router, with access to the database and session layer.
/// reference: https://github.com/maxcountryman/axum-login/blob/main/examples/sqlite/src/web/app.rs#L55
pub fn auth_router(pool: &DbPool, session_layer: &SessionManagerLayer<PostgresStore>) -> Router {
  let backend = Backend::new(pool.clone());
  let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer.clone()).build();

  Router::new()
    .route("/login", post(post::login))
    .route("/logout", post(post::logout))
    .layer(auth_layer)
    .route_layer(login_required!(Backend, login_url = "/login")) // routes after route layer will
                                                                 // not have middleware applied
}

/// Raise an error if user is not logged in
pub fn assert_authenticated(auth_session: &AuthSession) -> ApiResult<()> {
  if auth_session.user.is_none() {
    return Err(RouteError::Unauthorized.into());
  }
  Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Newtype since cannot derive traits for types defined in other crates
pub struct UserAuthWrapper(pub User);

impl From<User> for UserAuthWrapper {
  fn from(user: User) -> Self { Self(user) }
}

impl AuthUser for UserAuthWrapper {
  type Id = Uuid;

  fn id(&self) -> Self::Id { self.0.id }

  // todo: this should probably be a session cookie or something, not the password
  fn session_auth_hash(&self) -> &[u8] { self.0.password_hash.as_bytes() }
}

/// Form extractor for authentication fields.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
  pub username:     String,
  pub password:     String,
  /// where to redirect the user after login; i.e. the page they were trying to access
  pub redirect_url: Option<String>,
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
    let user =
      db::get_user_by_username(&self.pool, &credentials.username).await?.map(UserAuthWrapper::from);

    // Verifying the password is blocking and potentially slow, so use `spawn_blocking`.
    task::spawn_blocking(move || {
      Ok(user.filter(|user| user.0.verify_password(&credentials.password).is_ok()))
    })
    .await?
  }

  async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
    let user = db::get_user_by_id(&self.pool, *user_id).await?.map(UserAuthWrapper::from);
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

mod post {
  //! The login and logout handlers
  use super::*;

  /// User login. If successful, redirect to the next page, or to `/login` if no next page is
  /// provided.
  pub async fn login(
    mut auth_session: AuthSession,
    Form(creds): Form<Credentials>,
  ) -> impl IntoResponse {
    let login_url = match &creds.redirect_url {
      Some(next) => format!("/login?next={}", next),
      None => "/login".to_string(),
    };

    let user = match auth_session.authenticate(creds.clone()).await {
      Ok(Some(user)) => user,
      Ok(None) => {
        tracing::info!("Redirecting user {} to login", creds.username);
        return Redirect::to(&login_url).into_response();
      },
      Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
      tracing::error!("Failed to log in user {}", user.0.username);
      return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    tracing::info!("User {} is already logged in", user.0.username);
    Redirect::to(&login_url).into_response()
  }

  /// User logout. Redirect to `/login` on success, or return a 500 error on failure.
  pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.logout().await {
      Ok(_) => Redirect::to("/login").into_response(),
      Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
  }
}
