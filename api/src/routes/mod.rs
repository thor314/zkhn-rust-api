use axum::{routing, Json, Router};

use self::{comments::comments_router, openapi::docs_router, users::users_router};
use crate::SharedState;

// pub mod so that payloads and responses can be accessed by integration tests
pub mod comments;
pub mod items;
pub mod openapi;
pub mod user_votes;
pub mod users;

async fn health() -> &'static str { "ok" }

// todo: might have to move state into here
pub(crate) fn routes(state: SharedState) -> Router {
  Router::new()
    .route("/health", routing::get(health))
    .nest("/docs", docs_router())
    .nest("/users", users_router(state.clone()))
  // .merge(auth_router()) // todo(auth)
  // .nest("/items", items_router(state.clone()))
  // .nest("/comments", comments_router(state.clone()))
  // ..
}
