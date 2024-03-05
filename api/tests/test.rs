#![allow(unused_imports)]
#![allow(dead_code)]
mod common;

use std::borrow::BorrowMut;

use axum::http::{Request, StatusCode};
use common::*;
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test]
// https://github.com/launchbadge/sqlx/blob/main/examples/postgres/axum-social-with-tests/tests/user.rs
async fn test_create_user(pool: PgPool) {
  let mut app = api::api_router(&pool).await.expect("failed to build router");

  let resp1 = app
      .borrow_mut()
      // We handle JSON objects directly to sanity check the serialization and deserialization
      .oneshot(Request::post("/v1/user").json(json! {{
          "username": "alice",
          "password": "rustacean since 2015"
      }}))
      .await
      .unwrap();

  assert_eq!(resp1.status(), StatusCode::NOT_FOUND);
}
