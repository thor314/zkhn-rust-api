//! documentation for testing with sqlx: https://github.com/launchbadge/sqlx/blob/main/examples/postgres/axum-social-with-tests/tests/user.rs
//! documentation for testing with axum: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs

use std::borrow::{Borrow, BorrowMut};

use axum::{
  body::Body,
  extract::connect_info::MockConnectInfo,
  http::{self, Request, StatusCode},
};
use db::models::user::User;
use http_body_util::BodyExt; // for `collect`
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
use tracing::info;

use super::common::*;
use crate::{utils, ChangePasswordPayload, PasswordCreds, UserUpdatePayload};

#[sqlx::test(migrations = "../db/migrations")]
async fn simple_test_demo(pool: PgPool) {
  setup_test_tracing();
  let app = crate::router(&pool, None).await.expect("failed to build router");

  let get_request = Request::builder().uri("/health").body(Body::empty()).unwrap();
  let response = app.clone().oneshot(get_request).await.unwrap();
  // println!("response: {:?}", response);
  assert!(response.status().is_success());
  let response_body = response.into_body().collect().await.unwrap().to_bytes();
  assert_eq!(b"ok", &*response_body);

  let get_request = Request::builder().uri("/users/alice").body(Body::empty()).unwrap();
  let response = app.oneshot(get_request).await.unwrap();
  println!("response: {:?}", response);
  assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "../db/migrations")]
async fn test_user_crud_cycle(pool: PgPool) {
  setup_test_tracing();
  let app = crate::router(&pool, None).await.expect("failed to build router");

  let user_payload =
    crate::UserPayload::new("alice", "password", Some("email@email.com"), None).unwrap();

  let post_request = Request::builder().uri("/users").method("POST").json(json!(user_payload));
  let response = app.clone().oneshot(post_request).await.unwrap();
  // println!("response: {:?}", response);
  assert_eq!(response.status(), StatusCode::OK);

  let get_request = Request::builder().uri("/users/alice").body(Body::empty()).unwrap();
  let response = app.clone().oneshot(get_request).await.unwrap();
  // println!("response: {:?}", response);
  assert_eq!(response.status(), StatusCode::OK);
  let user = response.into_body().collect().await.unwrap().to_bytes();
  let user: User = serde_json::from_slice(&user).unwrap();
  // println!("user: {:?}", user.about);
  assert!(user.about.is_none() || user.about.as_ref().unwrap().0.is_empty());

  let update_payload =
    UserUpdatePayload::new("alice", Some("password"), Some("email@email.com"), Some("about"))
      .unwrap();
  let put = Request::builder().uri("/users").method("PUT").json(json!(update_payload));
  let response = app.clone().oneshot(put).await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);

  let get_request = Request::builder().uri("/users/alice").body(Body::empty()).unwrap();
  let response = app.clone().oneshot(get_request).await.unwrap();
  // println!("response: {:?}", response);
  assert_eq!(response.status(), StatusCode::OK);
  let user = response.into_body().collect().await.unwrap().to_bytes();
  let user: User = serde_json::from_slice(&user).unwrap();
  println!("user: {:?}", user.about);
  assert!(user.about.as_ref().unwrap().0 == "about");

  let delete = Request::builder().uri("/users/alice").method("DELETE").body(Body::empty()).unwrap();
  let response = app.clone().oneshot(delete).await.unwrap();
  // println!("response: {:?}", response);
  assert_eq!(response.status(), StatusCode::OK);

  let get_request = Request::builder().uri("/users/alice").body(Body::empty()).unwrap();
  let response = app.clone().oneshot(get_request).await.unwrap();
  // println!("response: {:?}", response);
  assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "../db/migrations")]
async fn test_user_login_logout(pool: PgPool) {
  let app = router_with_user_alice(&pool).await;

  let creds = PasswordCreds::new("alice", "password", None);
  let request = Request::builder().uri("/login/password").method("POST").json(json!(creds));
  let response = app.clone().oneshot(request).await.unwrap();
  dbg!(&response);
  assert!(response.status().is_redirection());
  let body = &response.into_body().collect().await.unwrap();
  dbg!(&body);
  panic!();

  // check double-login
  // let login_request =
  // Request::builder().uri("/login/password").method("POST").json(json!(creds)); let response =
  // app.clone().oneshot(login_request).await.unwrap(); let body =
  // &response.into_body().collect().await.unwrap(); assert_eq!(response.status(),
  // StatusCode::TEMPORARY_REDIRECT);

  // panic!();

  // let auth_session = AuthSession::
  // todo
  // let logout = Request::builder()
  //   .uri("/users/logout")
  //   .method("POST")
  //   .json(json!({"username": "alice"}));
}

#[sqlx::test(migrations = "../db/migrations")]
async fn test_request_password_reset_link(pool: PgPool) {
  let app = router_with_user_alice(&pool).await;

  //"/users/reset_password_link/:username", routing::put(put::request_password_reset_link))
  //"/users/reset_password_link/alice").method("PUT").empty_body();
  let request =
    Request::builder().uri("/users/reset-password-link/alice").method("PUT").empty_body();
  let response = app.clone().oneshot(request).await.unwrap();
  dbg!(&response);
  // assert!(response.status().is_success());
  let body = &response.into_body().collect().await.unwrap();
  dbg!(&body);
  panic!();

  let request =
    Request::builder().uri("/users/reset-password-link/alice").method("PUT").empty_body();
  let response = app.clone().oneshot(request).await.unwrap();
  assert!(response.status().is_success());
}

#[sqlx::test(migrations = "../db/migrations")]
async fn test_change_password(pool: PgPool) {
  let app = router_with_user_alice(&pool).await;

  // alice already has a password, this should fail
  let payload = json!(ChangePasswordPayload::new("alice", "password", "new_password").unwrap());
  let request = Request::builder().uri("/users/change-password").method("PUT").json(payload);
  let response = app.clone().oneshot(request).await.unwrap();
  // dbg!(&response);
  assert!(response.status().is_client_error()); // todo: granularity
  let body = &response.into_body().collect().await.unwrap();
  dbg!(&body);
  panic!();

  // change her password
  let payload = json!(ChangePasswordPayload::new("alice", "password", "new_password").unwrap());
  let request = Request::builder().uri("/users/change-password").method("PUT").json(payload);
  let response = app.clone().oneshot(request).await.unwrap();
  assert!(response.status().is_success()); // todo: granularity

  // change her password again
  let payload = json!(ChangePasswordPayload::new("alice", "new_password", "password").unwrap());
  let request = Request::builder().uri("/users/change-password").method("PUT").json(payload);
  let response = app.clone().oneshot(request).await.unwrap();
  assert!(response.status().is_success()); // todo: granularity
}
