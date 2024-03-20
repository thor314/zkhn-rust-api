//! documentation for testing with sqlx: https://github.com/launchbadge/sqlx/blob/main/examples/postgres/axum-social-with-tests/tests/user.rs
//! documentation for testing with axum: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod common;

use std::borrow::{Borrow, BorrowMut};

use api::UserUpdatePayload;
use axum::{
  body::Body,
  extract::connect_info::MockConnectInfo,
  http::{self, Request, StatusCode},
};
use common::*;
use db::models::user::User;
use http_body_util::BodyExt; // for `collect`
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
use tracing::info;

#[sqlx::test(migrations = "../db/migrations")]
async fn simple_test_demo(pool: PgPool) {
  let app = api::router(&pool, None).await.expect("failed to build router");

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
  let app = api::router(&pool, None).await.expect("failed to build router");

  let user_payload =
    api::UserPayload::new("alice", "password", Some("email@email.com"), None).unwrap();

  let post_request = Request::builder().uri("/users").method("POST").json(json!(user_payload));
  let response = app.clone().oneshot(post_request).await.unwrap();
  // println!("response: {:?}", response);
  assert_eq!(response.status(), StatusCode::CREATED);

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
  let app = api::router(&pool, None).await.expect("failed to build router");
  let user_payload =
    api::UserPayload::new("alice", "password", Some("email@email.com"), None).unwrap();

  let post_request = Request::builder().uri("/users").method("POST").json(json!(user_payload));
  let response = app.clone().oneshot(post_request).await.unwrap();
  // println!("response: {:?}", response);
  assert_eq!(response.status(), StatusCode::CREATED);

  let login = Request::builder()
    .uri("/users/login")
    .method("POST")
    .json(json!({"username": "alice", "password": "password"}));
  let response = app.clone().oneshot(login).await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);

  // todo
  // let logout = Request::builder()
  //   .uri("/users/logout")
  //   .method("POST")
  //   .json(json!({"username": "alice"}));

}