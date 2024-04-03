//! documentation for testing with sqlx: https://github.com/launchbadge/sqlx/blob/main/examples/postgres/axum-social-with-tests/tests/user.rs
//! documentation for testing with axum: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs

use axum::http::StatusCode;
use sqlx::PgPool;
use tower_cookies::Key;

use crate::{
  routes::users::{payload::UserUpdatePayload, ChangePasswordPayload, UserPayload},
  tests::common::{jsend, router_with_user_alice, send, setup_test_tracing},
  CredentialsPayload,
};

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
  // debug:
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
