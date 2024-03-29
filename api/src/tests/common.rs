use axum::{
  body::Body,
  http::{request, Request},
  Router,
};
use reqwest::StatusCode;
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

use crate::routes::users::UserPayload;

static INIT: std::sync::Once = std::sync::Once::new();
/// Set up tracing for a test. Avoid duplicate work.
pub fn setup_test_tracing() {
  use tracing::Level;
  use tracing_subscriber::FmtSubscriber;

  INIT.call_once(|| {
    let subscriber =
      FmtSubscriber::builder().with_max_level(Level::DEBUG).with_test_writer().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
  });
}

/// convenience methods for building requests
pub trait RequestBuilderExt {
  fn json(self, json: serde_json::Value) -> Request<Body>;

  fn empty_body(self) -> Request<Body>;
}

impl RequestBuilderExt for request::Builder {
  fn json(self, json: serde_json::Value) -> Request<Body> {
    self
      .header("Content-Type", "application/json")
      .body(Body::from(json.to_string()))
      .expect("failed to build request")
  }

  fn empty_body(self) -> Request<Body> {
    self.body(Body::empty()).expect("failed to build request")
  }
}

/// create a router with a user named "alice".
pub async fn router_with_user_alice(pool: PgPool) -> Router {
  let app = crate::app(pool).await.expect("failed to build router");

  let user_payload = UserPayload::new("alice", "password", Some("email@email.com"), None).unwrap();

  let post_request = Request::builder().uri("/users").method("POST").json(json!(user_payload));
  let response = app.clone().oneshot(post_request).await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);

  setup_test_tracing();
  app
}
