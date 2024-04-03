use axum::{
  body::{self, Body},
  http::{request, Request, Response},
  Router,
};
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
use tower_cookies::Key;
use tracing_subscriber::EnvFilter;

use crate::routes::users::UserPayload;

static INIT: std::sync::Once = std::sync::Once::new();
/// Set up tracing for a test. Avoid duplicate work.
pub fn setup_test_tracing() {
  use tracing::Level;
  use tracing_subscriber::FmtSubscriber;

  INIT.call_once(|| {
    let filter = EnvFilter::from_default_env()
      .add_directive("api=debug".parse().unwrap())
      .add_directive("db=debug".parse().unwrap())
      .add_directive("server=debug".parse().unwrap())
      .add_directive("sqlx=info".parse().unwrap());
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).with_test_writer().finish();
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
  let app = crate::app(pool, Key::generate()).await.expect("failed to build router");

  let user_payload = UserPayload::new("alice", "password", Some("email@email.com"), None).unwrap();

  let post_request = Request::builder().uri("/users").method("POST").json(json!(user_payload));
  let response = app.clone().oneshot(post_request).await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);

  setup_test_tracing();
  app
}

/// convenience method to send a json payload to a route and assert the status code
pub async fn jsend<P: Serialize>(
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
pub async fn send(
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
