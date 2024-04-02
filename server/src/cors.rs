use axum::http::{self, HeaderValue, Method};
use tower_http::cors::CorsLayer;

// see https://docs.rs/tower-http/latest/tower_http/cors/index.html
// for more details
//
// pay attention that for some request types like posting content-type: application/json
// it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
// or see this issue https://github.com/tokio-rs/axum/issues/849
/// ref: https://github.com/tokio-rs/axum/blob/d703e6f97a0156177466b6741be0beac0c83d8c7/examples/cors/src/main.rs#L31
pub(super) fn cors_layer() -> CorsLayer {
  let origins: Vec<_> = ["http://localhost:3000", "http://localhost:8000"]
    .into_iter()
    .map(|origin| origin.parse::<HeaderValue>().unwrap())
    .collect();

  CorsLayer::new()
  // todo(prod): set to env var setting dev/prod 
    .allow_origin(origins)
    .allow_headers([http::header::CONTENT_TYPE]) // allow json headers
    .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
    .allow_credentials(true)
}
