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

use crate::{
  // auth::credentials::password_creds::PasswordCreds,
  routes::users::{payload::UserUpdatePayload, ChangePasswordPayload, UserPayload},
  tests::common::{router_with_user_alice, setup_test_tracing, RequestBuilderExt},
};

#[sqlx::test(migrations = "../db/migrations")]
async fn test_user_crud_cycle(pool: PgPool) {
  setup_test_tracing();
  let app = crate::router(&pool, None).await.expect("failed to build router");

  let user_payload = UserPayload::new("alice", "password", Some("email@email.com"), None).unwrap();

  let request = Request::builder().uri("/users").method("POST").json(json!(user_payload));
  let response = app.clone().oneshot(request).await.unwrap();
  // println!("response: {:?}", response);
  assert_eq!(response.status(), StatusCode::OK);

  let request = Request::builder().uri("/users/alice").body(Body::empty()).unwrap();
  let response = app.clone().oneshot(request).await.unwrap();
  // println!("response: {:?}", response);
  assert_eq!(response.status(), StatusCode::OK);
  let user = response.into_body().collect().await.unwrap().to_bytes();
  let user: User = serde_json::from_slice(&user).unwrap();
  // println!("user: {:?}", user.about);
  assert!(user.about.is_none() || user.about.as_ref().unwrap().0.is_empty());

  let update_payload =
    UserUpdatePayload::new("alice", Some("newemail@email.com"), Some("about")).unwrap();
  let request =
    Request::builder().uri("/users/about").method("PUT").json(json!(update_payload.clone()));
  let response = app.clone().oneshot(request).await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);

  let request = Request::builder().uri("/users/email").method("PUT").json(json!(update_payload));
  let response = app.clone().oneshot(request).await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);

  let request = Request::builder().uri("/users/alice").method("GET").body(Body::empty()).unwrap();
  let response = app.clone().oneshot(request).await.unwrap();
  // println!("response: {:?}", response);
  assert_eq!(response.status(), StatusCode::OK);
  let user = response.into_body().collect().await.unwrap().to_bytes();
  let user: User = serde_json::from_slice(&user).unwrap();
  println!("user: {:?}", user.about);
  assert!(user.about.as_ref().unwrap().0 == "about");
  assert!(user.email.as_ref().unwrap().0 == "newemail@email.com");

  let request =
    Request::builder().uri("/users/alice").method("DELETE").body(Body::empty()).unwrap();
  let response = app.clone().oneshot(request).await.unwrap();
  // println!("response: {:?}", response);
  assert_eq!(response.status(), StatusCode::OK);

  let request = Request::builder().uri("/users/alice").body(Body::empty()).unwrap();
  let response = app.clone().oneshot(request).await.unwrap();
  // println!("response: {:?}", response);
  assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// #[sqlx::test(migrations = "../db/migrations")]
// async fn test_user_login_logout(pool: PgPool) {
//   let app = router_with_user_alice(&pool).await;

//   let creds = PasswordCreds::new("alice", "password", None);
//   let request = Request::builder().uri("/login/password").method("POST").json(json!(creds));
//   let response = app.clone().oneshot(request).await.unwrap();
//   dbg!(&response);
//   assert!(response.status().is_redirection());
//   let body = &response.into_body().collect().await.unwrap();
//   dbg!(&body);
//   panic!();

//   // check double-login
//   // let login_request =
//   // Request::builder().uri("/login/password").method("POST").json(json!(creds)); let response =
//   // app.clone().oneshot(login_request).await.unwrap(); let body =
//   // &response.into_body().collect().await.unwrap(); assert_eq!(response.status(),
//   // StatusCode::TEMPORARY_REDIRECT);

//   // panic!();

//   // let auth_session = AuthSession::
//   // todo
//   // let logout = Request::builder()
//   //   .uri("/users/logout")
//   //   .method("POST")
//   //   .json(json!({"username": "alice"}));
// }

#[sqlx::test(migrations = "../db/migrations")]
async fn test_request_password_reset_link(pool: PgPool) {
  let app = router_with_user_alice(&pool).await;

  let request =
    Request::builder().uri("/users/reset-password-link/alice").method("PUT").empty_body();
  let response = app.clone().oneshot(request).await.unwrap();
  dbg!(&response);
  assert_eq!(response.status(), StatusCode::OK);
  let body = &response.into_body().collect().await.unwrap();
  dbg!(body);

  let request =
    Request::builder().uri("/users/reset-password-link/alice").method("PUT").empty_body();
  let response = app.clone().oneshot(request).await.unwrap();
  dbg!(&response);
  assert_eq!(response.status(), StatusCode::OK);
}

#[sqlx::test(migrations = "../db/migrations")]
async fn test_change_password(pool: PgPool) {
  let app = router_with_user_alice(&pool).await;

  // change her password
  let payload = json!(ChangePasswordPayload::new("alice", "password", "new_password").unwrap());
  let request = Request::builder().uri("/users/change-password").method("PUT").json(payload);
  let response = app.clone().oneshot(request).await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);
  // let body = &response.into_body().collect().await.unwrap();

  // change her password again
  let payload = json!(ChangePasswordPayload::new("alice", "new_password", "password").unwrap());
  let request = Request::builder().uri("/users/change-password").method("PUT").json(payload);
  let response = app.clone().oneshot(request).await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);
}
