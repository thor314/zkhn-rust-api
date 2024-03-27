use utoipa::OpenApi;
use utoipauto::utoipauto;

use super::users::{delete::*, get::*, post::*, put::*};
use crate::error::ApiError;

// ref: https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs#L22
#[utoipauto(paths = "./api/src/routes/users/mod.rs")] // auto-detect api paths
#[derive(OpenApi)]
#[openapi(
  info(description = "API documentation for ZKHN"),
  // 
  // Schemas that may be returned by the api
  components(schemas(ApiError))
  // runtime modification, e.g. for jwt: https://docs.rs/utoipa/latest/utoipa/trait.Modify.html
  // modifiers(..) 
  tags( (name = "zkhn-rust-api", description = "API for ZKHN") )
)]
struct ApiDoc;
