//! documentation for testing with sqlx: https://github.com/launchbadge/sqlx/blob/main/examples/postgres/axum-social-with-tests/tests/user.rs
//! documentation for testing with axum: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod common;

use std::borrow::{Borrow, BorrowMut};

use axum::{
  body::Body,
  http::{Request, StatusCode},
};
use common::*;
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
use tracing::info;

#[sqlx::test]
async fn simple_test_demo(pool: PgPool) {
  let app = api::router(&pool, None).await.expect("failed to build router");

  let get_request = Request::builder().uri("/users/username/alice").body(Body::empty()).unwrap();
  let response = app.oneshot(get_request).await.unwrap();
  println!("response: {:?}", response);
  assert!(response.status().is_server_error());
}

#[sqlx::test]
async fn test_create_user(pool: PgPool) {
  let app = api::router(&pool, None).await.expect("failed to build router");

  let user_payload = api::UserPayload::new("alice", "password", "email", None);
  let user_payload = serde_json::to_value(user_payload).unwrap();
  println!("user_payload: {:?}", user_payload);

  let put_request = Request::builder().uri("/users").method("PUT").json(user_payload);

  let response = app.oneshot(put_request).await.unwrap();
  println!("response: {:?}", response);
  assert!(response.status().is_success());
}
