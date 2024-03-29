//! documentation for testing with sqlx: https://github.com/launchbadge/sqlx/blob/main/examples/postgres/axum-social-with-tests/tests/user.rs
//! documentation for testing with axum: https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs

use std::{
  borrow::{Borrow, BorrowMut},
  error::Error,
};

use axum::{
  body::Body,
  extract::connect_info::MockConnectInfo,
  http::{self, Request, StatusCode},
  Router,
};
use axum_login::{login_required, AuthManagerLayerBuilder};
use db::models::user::User;
use http_body_util::BodyExt; // for `collect`
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
use tracing::info;

use crate::{auth::CredentialsPayload, error::ApiError};
use crate::{
  // auth::credentials::password_creds::PasswordCreds,
  routes::users::{payload::UserUpdatePayload, ChangePasswordPayload, UserPayload},
  tests::common::{router_with_user_alice, setup_test_tracing, RequestBuilderExt},
};

#[sqlx::test(migrations = "../db/migrations")]
async fn test_user_crud_cycle(pool: PgPool) {
  setup_test_tracing();
  let app = crate::app(pool).await.expect("failed to build router");

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
  let request = Request::builder().uri("/users").method("PUT").json(json!(update_payload.clone()));
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

#[sqlx::test(migrations = "../db/migrations")]
async fn test_request_password_reset_link(pool: PgPool) {
  let app = router_with_user_alice(pool).await;

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
  let app = router_with_user_alice(pool).await;

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

// todo: how to check cookies?
// ref: https://github.com/maxcountryman/axum-login/blob/main/axum-login/tests/integration-test.rs#L95
// assert!(res
//     .cookies()
//     .find(|c| c.name() == "id")
//     .is_some_and(|c| c.value() == ""));
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
  let cookies = login_response.headers();
  assert_eq!(login_response.status(), StatusCode::SEE_OTHER);

  let invalid_login_request = make_login_request("ferris", "password");
  let login_response = app.clone().oneshot(invalid_login_request).await.unwrap();
  dbg!(&login_response);
  assert_eq!(login_response.status(), StatusCode::UNAUTHORIZED);

  let logout_request = Request::builder().uri("/users/logout").method("POST").empty_body();
  let logout_response = app.clone().oneshot(logout_request).await.unwrap();
  dbg!(&logout_response);
  assert_eq!(logout_response.status(), StatusCode::SEE_OTHER);

  // let logout_request = Request::builder().uri("/users/logout").method("POST").empty_body();
  // let logout_response = app.clone().oneshot(logout_request).await.unwrap();
  // dbg!(&logout_response);
  // assert_eq!(logout_response.status(), StatusCode::SEE_OTHER);
}
