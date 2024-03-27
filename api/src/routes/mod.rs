use axum::{routing, Json, Router};
use utoipa::OpenApi;

use self::{comments::comments_router, users::users_router};
use crate::SharedState;

// pub mod so that payloads and responses can be accessed by integration tests
pub mod comments;
pub mod items;
pub mod openapi;
pub mod user_votes;
pub mod users;

async fn health() -> &'static str { "ok" }

// async fn openapi() -> Json<utoipa::openapi::OpenApi> { Json(ApiDoc::openapi()) }

// todo: might have to move state into here
pub(crate) fn routes(state: SharedState) -> Router {
  Router::new()
    .route("/health", routing::get(health))
    // .route("/openapi", routing::get(openapi))
    .nest("/users", users_router(state.clone()))
  // .nest("/items", items_router(state.clone()))
  // .nest("/comments", comments_router(state.clone()))
  // ..
}

// #[derive(OpenApi)]
// #[openapi(
//       paths(
//       // crate::routes::users::post::create_user
//       users::post::create_user,
//       ),
//       info(description = "My Api description"),
//     )]
// pub struct ApiDoc;
