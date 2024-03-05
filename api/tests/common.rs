#![allow(dead_code)]
use axum::{
  body::Body,
  http::{request, Request},
};

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
