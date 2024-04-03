//! documentation for testing with sqlx: https://github.com/launchbadge/sqlx/blob/main/examples/postgres/axum-social-with-tests/tests/user.rs
//! documentation for testing with axum: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs

use std::{
  borrow::{Borrow, BorrowMut},
  error::Error,
};

use axum::{
  body::{self, Body},
  extract::connect_info::MockConnectInfo,
  http::{self, Request, Response, StatusCode},
  Router,
};
use axum_login::{login_required, AuthManagerLayerBuilder};
use db::models::user::User;
use http_body_util::BodyExt; // for `collect`
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
use tower_cookies::Key;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
use tracing::info;

use crate::{
  error::ApiError,
  routes::users::{payload::UserUpdatePayload, ChangePasswordPayload, UserPayload},
  tests::common::{router_with_user_alice, setup_test_tracing, RequestBuilderExt},
  CredentialsPayload,
};

/// convenince method to send a json payload to a route and assert the status code
async fn jsend<P: Serialize>(
  app: &Router,
  payload: P,
  method: &str,
  uri: &str,
  status_code: StatusCode,
) -> Response<body::Body> {
  let request = Request::builder().uri(uri).method(method).json(json!(payload));
  let response = app.clone().oneshot(request).await.unwrap();
  assert_eq!(response.status(), status_code);
  response
}
/// convenince method to send an empty body to a route and assert the status code
async fn send(
  app: &Router,
  method: &str,
  uri: &str,
  status_code: StatusCode,
) -> Response<body::Body> {
  let request = Request::builder().uri(uri).method(method).body(Body::empty()).unwrap();
  let response = app.clone().oneshot(request).await.unwrap();
  assert_eq!(response.status(), status_code);
  response
}

// demo: how to collect body into a type
// -------------------------------------
// let user = _response.into_body().collect().await.unwrap().to_bytes();
// let user: User = serde_json::from_slice(&user).unwrap();

#[sqlx::test(migrations = "../db/migrations")]
async fn test_user_crud_cycle(pool: PgPool) {
  setup_test_tracing();
  let app = crate::app(pool, Key::generate()).await.expect("failed to build router");
  jsend(&app, UserPayload::default(), "POST", "/users", StatusCode::OK).await;
  // fail on duplicate create user
  jsend(&app, UserPayload::default(), "POST", "/users", StatusCode::CONFLICT).await;
  jsend(&app, CredentialsPayload::default(), "POST", "/users/login", StatusCode::OK).await;
  jsend(&app, UserUpdatePayload::default(), "PUT", "/users", StatusCode::OK).await;
  send(&app, "GET", "/users/alice", StatusCode::OK).await;
  send(&app, "DELETE", "/users/alice", StatusCode::OK).await;
  send(&app, "GET", "/users/alice", StatusCode::NOT_FOUND).await;
}

#[sqlx::test(migrations = "../db/migrations")]
async fn test_request_password_reset_link(pool: PgPool) {
  let app = router_with_user_alice(pool).await;
  jsend(&app, CredentialsPayload::default(), "POST", "/users/login", StatusCode::OK).await;
  send(&app, "PUT", "/users/reset-password-link/alice", StatusCode::OK).await;
  send(&app, "PUT", "/users/reset-password-link/alice", StatusCode::OK).await;
}

#[sqlx::test(migrations = "../db/migrations")]
async fn test_change_password(pool: PgPool) {
  let app = router_with_user_alice(pool).await;
  jsend(&app, CredentialsPayload::default(), "POST", "/users/login", StatusCode::OK).await;
  jsend(&app, ChangePasswordPayload::default(), "PUT", "/users/change-password", StatusCode::OK)
    .await;
  let new_payload = ChangePasswordPayload::new("alice", "new_password", "password").unwrap();
  jsend(&app, new_payload, "PUT", "/users/change-password", StatusCode::OK).await;
  send(&app, "POST", "/users/logout", StatusCode::OK).await;
}
