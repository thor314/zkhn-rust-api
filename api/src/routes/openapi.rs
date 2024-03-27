use axum::{routing, Json, Router};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;

use super::users::{delete::*, get::*, post::*, put::*, *};
use crate::error::ApiError;

// ref: https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs#L22
#[utoipauto(paths = "./api/src/routes/users/mod.rs")] // auto-detect api paths
#[derive(OpenApi)]
#[openapi(
  info(description = "API documentation for ZKHN"),
  // Schemas that may be returned in the body by the api.
  components(schemas(ApiError, UserResponse))
  // runtime modification, e.g. for jwt: https://docs.rs/utoipa/latest/utoipa/trait.Modify.html
  // low-priority, but could gate moderator methods with an auth token.
  // modifiers(..) 
  tags( (name = "zkhn-rust-api", description = "API for ZKHN") )
)]
pub(super) struct ApiDoc;

/// Return JSON version of an OpenAPI schema.
/// SwaggerUI and RapiDoc use this to generate documentation.
#[utoipa::path(
    get,
    path = "/docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
pub(super) async fn openapi_docs() -> Json<utoipa::openapi::OpenApi> { Json(ApiDoc::openapi()) }

/// router fragment, supplying OpenAPI documentation
/// View swagger at: http://localhost:3000/docs/swagger-ui
/// View rapidoc at: http://localhost:3000/docs/rapidoc
pub(super) fn docs_router() -> Router {
  Router::new()
    .route("/openapi.json", routing::get(openapi_docs))
    .merge(SwaggerUi::new("/swagger-ui").url("/docs/openapi.json", ApiDoc::openapi()))
    .merge(RapiDoc::new("/docs/openapi.json").path("/rapidoc"))
}
