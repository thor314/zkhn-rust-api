//! notes on utoipa:
//! Style guide for documenting a route:
//! #[utoipa::path(
//!   get|post|put|delete|etc,
//!   path = "/path/to/route", e.g. "/users", or "/users/{username}",
//! <if path param>
//!   params( ("username" = String, Path, example = "alice") )[^1][^2]
//! <if json param>
//!   params( ("payload" = MyPayload, example = json!(Payload::default())) )[^3]
//! <if response body is json>
//!   responses(
//!     (status = 404, description = "Not Found"), // don't include body for errors
//!     (status = 200, description = "Success", body = UserResponse),
//!     ...
//!   )
//! ]
//! [^1]: If param is a newtype like `Username(String)`, use the underlying type `String` for the example for clarity.
//! [^2]: example fields make testing the api in the UI more convenient.
//! [^3]: Try to provide defaults for the payload struct; use `json!` to create an example.
//!
//! Derive IntoParams for Payloads sent as Path or Query params.
//! Derive ToSchema for Payloads and Responses.
use axum::{routing, Json, Router};
use db::models::user::User;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipauto::utoipauto;

// use super::items::{get::*, post::*, put::*, *};
use super::items::*;
use super::users::{get::*, post::*, put::*, *};

/// router fragment supplying OpenAPI documentation and ui routes
/// View rapidoc documentation page at: http://localhost:3000/docs/rapidoc
pub(super) fn docs_router() -> Router {
  Router::new()
    .route("/openapi.json", routing::get(openapi_docs))
    .merge(RapiDoc::new("/docs/openapi.json").path("/rapidoc"))
}

// ref: https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs#L22
#[utoipauto(paths = "./api/src/routes/users/mod.rs")] // auto-detect api paths
#[derive(OpenApi)]
#[openapi(
  info(description = "API documentation for ZKHN"),
  // Schemas that may be returned in the body by the api.
  components(schemas(
    User, UserUpdatePayload, ChangePasswordPayload, CreateUserPayload,
    CredentialsPayload, GetUserResponse, CreateUserResponse, AuthenticateUserResponse, AuthUserResponseInternal,
    CreateItemPayload,
    GetItemResponse, GetEditItemResponse, GetDeleteItemResponse,
    VotePayload, VotePayloadEnum, FavoritePayload, FavoritePayloadEnum, HiddenPayload, HiddenPayloadEnum))
  // runtime modification, e.g. for jwt: https://docs.rs/utoipa/latest/utoipa/trait.Modify.html
  // low-priority, but could gate moderator methods with an auth token.
  // modifiers(..) 
  tags( (name = "zkhn-rust-api", description = "API for ZKHN") )
)]
struct ApiDoc;

/// Return JSON version of an OpenAPI schema.
/// RapiDoc uses this to generate documentation.
#[utoipa::path(
    get,
    path = "/docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
async fn openapi_docs() -> Json<utoipa::openapi::OpenApi> { Json(ApiDoc::openapi()) }
