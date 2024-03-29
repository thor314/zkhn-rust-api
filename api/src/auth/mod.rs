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
//! there are 2 main traits to implement:
//! - AuthUser - Implement for a user that can be authenticated and authorized. Requires methods for
//!   getting the user's unique id and the password hash or access token.
//! - AuthnBackend - given a user_id, get the user from the database
//!   - session_auth_hash, which is used to validate the session; provide some credentials to
//!     validate the session
//!   - get_user, which is used to load the user from the backend into the session.

#[cfg(test)] mod test;
mod users;
mod web;

use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use db::DbPool;
use tower_sessions::service::SignedCookie;
use tower_sessions_sqlx_store::PostgresStore;

pub(crate) use self::users::{AuthBackend, AuthSession};
use crate::sessions::MySessionManagerLayer;

pub type MyAuthLayer = AuthManagerLayer<AuthBackend, PostgresStore, SignedCookie>;

pub fn get_auth_layer(pool: DbPool, session_layer: MySessionManagerLayer) -> MyAuthLayer {
  let backend = AuthBackend::new(pool);
  AuthManagerLayerBuilder::new(backend, session_layer).build()
}
