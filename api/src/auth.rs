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
//!
//! Axum-login cheatsheet
//! ---------------------
//!
//! there are 3 main traits to implement:
//! - AuthUser - state the user struct, and provide methods to get the id and session_auth_hash
//! - AuthnBackend - given a user_id, get the user from the database
//!   - session_auth_hash, which is used to validate the session; provide some credentials to
//!     validate the session
//!   - get_user, which is used to load the user from the backend into the session.

use std::fmt;

use anyhow::Context;
use axum::{
  async_trait,
  http::{
    header::{AUTHORIZATION, USER_AGENT},
    StatusCode,
  },
  response::{IntoResponse, Redirect},
  routing, Form, Router,
};
use axum_login::{
  login_required,
  tower_sessions::{session, SessionStore},
  AuthManagerLayerBuilder, AuthUser, AuthnBackend, AuthzBackend, UserId,
};
use db::{
  models::user::User, password::verify_user_password, queries, AuthToken, Password, Username,
};
use oauth2::{
  basic::{BasicClient, BasicRequestTokenError},
  reqwest::{async_http_client, AsyncHttpClientError},
  url::Url,
  AuthorizationCode, CsrfToken, TokenResponse,
};
use serde::{Deserialize, Serialize};
use tokio::task;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
use uuid::Uuid;

use crate::{error::ApiError, ApiResult, DbPool, SharedState};

/// Axum extractor for the current user session.
pub type AuthSession = axum_login::AuthSession<Backend>;

/// construct the auth router, with access to the database and session layer.
/// reference: https://github.com/maxcountryman/axum-login/blob/main/examples/sqlite/src/web/app.rs#L55
pub fn auth_router(pool: &DbPool, session_layer: &SessionManagerLayer<PostgresStore>) -> Router {
  // let backend = Backend::new(pool.clone(), BasicClient::new(
    // "client_id".to_string()));
  // let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer.clone()).build();

  // Router::new()
  //   .route("/login", routing::post(post::login))
  //   .route("/logout", routing::post(post::logout))
  //   .layer(auth_layer)
  //   .route_layer(login_required!(Backend, login_url = "/login")) // routes after route layer will
  //                                                                // not have middleware applied
  todo!()
}

/// Raise an error if user is not logged in
pub fn assert_authenticated(auth_session: &AuthSession) -> ApiResult<()> {
  if auth_session.user.is_none() {
    return Err(ApiError::Unauthorized("user is not logged in".to_string()));
  }
  Ok(())
}

/// Newtype Wrapper allows us to derive `AuthUser` for `User`, despite `User` living in `db`.`
#[derive(Clone, Serialize, Deserialize)]
pub struct UserAuthWrapper(pub User);

// explicitly implemented to intentionally redact sensitive information
impl fmt::Debug for UserAuthWrapper {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("User").field("username", &self.0.username).finish()
  }
}

impl From<User> for UserAuthWrapper {
  fn from(user: User) -> Self { Self(user) }
}

impl AuthUser for UserAuthWrapper {
  type Id = Username;

  fn id(&self) -> Self::Id { self.0.username.clone() }

  fn session_auth_hash(&self) -> &[u8] {
    self.0.auth_token.as_ref().expect("expected auth token").0.as_bytes()
  }
}

// todo: kill
// /// Form extractor for authentication fields.
// ///
// /// This allows us to extract the authentication fields from forms.
// /// We use this to authenticate requests with the backend.
// #[derive(Debug, Clone, Deserialize)]
// pub struct Credentials {
//   pub username:     Username,
//   pub password:     Password,
//   /// where to redirect the user after login; i.e. the page they were trying to access
//   pub redirect_url: Option<String>,
// }
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
  pub code:      String,
  pub old_state: CsrfToken,
  pub new_state: CsrfToken,
}

#[derive(Debug, Deserialize)]
struct UserInfo {
  login: String,
}

#[derive(Clone)]
pub struct Backend {
  pool:   DbPool,
  // client to interact with Oauth endpoints
  client: BasicClient,
}

impl Backend {
  pub fn new(pool: DbPool, client: BasicClient) -> Self { Self { pool, client } }

  pub fn authorize_url(&self) -> (Url, CsrfToken) {
    self.client.authorize_url(CsrfToken::new_random).url()
  }
}

#[async_trait]
impl AuthnBackend for Backend {
  type Credentials = Credentials;
  type Error = ApiError;
  type User = UserAuthWrapper;

  // verify that the CRSF state has not been tampered with
  // then process the authorization code, expecting a token response
  // then
  async fn authenticate(
    &self,
    creds: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    // Ensure the CSRF state has not been tampered with.
    if creds.old_state.secret() != creds.new_state.secret() {
      return Ok(None);
    }

    // Process authorization code, expecting a token response back.
    let token_res = self
      .client
      .exchange_code(AuthorizationCode::new(creds.code))
      .request_async(async_http_client)
      .await
      .map_err(Self::Error::OAuth2)?;

    // Use access token to request user info.
    // See: https://docs.github.com/en/rest/overview/resources-in-the-rest-api?apiVersion=2022-11-28#user-agent-required
    let user_info = reqwest::Client::new()
      .get("https://api.github.com/user")
      .header(USER_AGENT.as_str(), "axum-login")
      .header(AUTHORIZATION.as_str(), format!("Bearer {}", token_res.access_token().secret()))
      .send()
      .await
      .map_err(Self::Error::AuthReqwest)?
      .json::<UserInfo>()
      .await
      .map_err(Self::Error::AuthReqwest)?;

    // store the access token
    // let user = db::update_user_auth_token(&self.pool, &creds.username,
    // &AuthToken(*token_res.access_token().secret())).await?;
    let user = todo!();
    Ok(Some(user))
  }

  async fn get_user(&self, username: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
    let user = db::queries::get_user(&self.pool, &username.0).await?.map(UserAuthWrapper::from);
    Ok(user)
  }
}

// todo: below this line must this all die
// ---------------------------------------
// 
// mod post {
//   //! The login and logout handlers
//   use super::*;

//   /// User login.
//   /// If successful, redirect to `credentials.next_url`, or else home.
//   pub async fn login(
//     mut auth_session: AuthSession,
//     Form(creds): Form<Credentials>,
//   ) -> ApiResult<impl IntoResponse> {
//     let login_url = match &creds.redirect_url {
//       Some(next) => format!("/login?next={}", next),
//       None => "/login".to_string(),
//     };

//     let user = match auth_session.authenticate(creds.clone()).await {
//       Ok(Some(user)) => user,
//       Ok(None) => {
//         tracing::info!("Redirecting user {} to login", creds.username);
//         return Ok(Redirect::to(&login_url).into_response());
//       },
//       Err(_) =>
//         return Err(ApiError::AuthenticationError("failed to authenticate user".to_string())),
//     };

//     if auth_session.login(&user).await.is_err() {
//       return Err(ApiError::AuthenticationError("failed to log in user".to_string()));
//     }

//     tracing::info!("User {} is already logged in", user.0.username);
//     Ok(Redirect::to(&login_url).into_response())
//   }

//   /// User logout. Redirect to `/login` on success, or return a 500 error on failure.
//   pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
//     match auth_session.logout().await {
//       Ok(_) => Redirect::to("/login").into_response(),
//       Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     }
//   }
// }
