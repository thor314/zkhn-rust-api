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
  let _r = jsend(&app, UserPayload::default(), "POST", "/users", StatusCode::OK).await;
  // fail on duplicate create user
  let _r = jsend(&app, UserPayload::default(), "POST", "/users", StatusCode::CONFLICT).await;
  let _r = jsend(&app, CredentialsPayload::default(), "POST", "/users/login", StatusCode::OK).await;
  let _r = jsend(&app, UserUpdatePayload::default(), "PUT", "/users", StatusCode::OK).await;
  let _r = send(&app, "GET", "/users/alice", StatusCode::OK).await;
  let _r = send(&app, "DELETE", "/users/alice", StatusCode::OK).await;
  let _r = send(&app, "GET", "/users/alice", StatusCode::NOT_FOUND).await;
}

#[sqlx::test(migrations = "../db/migrations")]
async fn test_request_password_reset_link(pool: PgPool) {
  let app = router_with_user_alice(pool).await;
  let _r = jsend(&app, CredentialsPayload::default(), "POST", "/users/login", StatusCode::OK).await;
  let _r = send(&app, "PUT", "/users/reset-password-link/alice", StatusCode::OK).await;
  let _r = send(&app, "PUT", "/users/reset-password-link/alice", StatusCode::OK).await;
}

#[sqlx::test(migrations = "../db/migrations")]
async fn test_change_password(pool: PgPool) {
  let app = router_with_user_alice(pool).await;

  // login
  let _response =
    jsend(&app, CredentialsPayload::default(), "POST", "/users/login", StatusCode::OK).await;

  // change her password
  let _response =
    jsend(&app, ChangePasswordPayload::default(), "PUT", "/users/change-password", StatusCode::OK)
      .await;
  // let body = &response.into_body().collect().await.unwrap();

  // change her password again
  let _response = jsend(
    &app,
    ChangePasswordPayload::new("alice", "new_password", "password").unwrap(),
    "PUT",
    "/users/change-password",
    StatusCode::OK,
  )
  .await;
}

#[sqlx::test(migrations = "../db/migrations")]
async fn test_login_logout(pool: PgPool) {
  let app = router_with_user_alice(pool).await;

  let make_login_request = |username: &str, password: &str| {
    let credentials = CredentialsPayload::new(username, password, None);
    Request::builder().uri("/users/login").method("POST").json(json!(credentials))
  };

  let valid_login_request = make_login_request("alice", "password");
  let login_response = app.clone().oneshot(valid_login_request).await.unwrap();
  dbg!(&login_response);
  assert_eq!(login_response.status(), StatusCode::OK);

  let invalid_login_request = make_login_request("ferris", "password");
  let login_response = app.clone().oneshot(invalid_login_request).await.unwrap();
  dbg!(&login_response);
  assert_eq!(login_response.status(), StatusCode::UNAUTHORIZED);

  let logout_request = Request::builder().uri("/users/logout").method("POST").empty_body();
  let logout_response = app.clone().oneshot(logout_request).await.unwrap();
  dbg!(&logout_response);
  assert_eq!(logout_response.status(), StatusCode::OK);
}
