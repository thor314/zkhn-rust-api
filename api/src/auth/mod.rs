//! Authentication with axum-login.

mod users;
mod web;

use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use db::DbPool;
use tower_sessions::service::SignedCookie;
use tower_sessions_sqlx_store::PostgresStore;

pub(crate) use self::{
  users::{AuthBackend, AuthSession, CredentialsPayload},
  web::{login_post_internal, logout_post_internal},
};
use crate::sessions::MySessionManagerLayer;

pub type MyAuthLayer = AuthManagerLayer<AuthBackend, PostgresStore, SignedCookie>;

pub fn get_auth_layer(pool: DbPool, session_layer: MySessionManagerLayer) -> MyAuthLayer {
  let backend = AuthBackend::new(pool);
  AuthManagerLayerBuilder::new(backend, session_layer).build()
}
