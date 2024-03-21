use axum::{routing, Router};

use self::users::users_router;
use crate::SharedState;

pub mod comments;
pub mod items;
pub mod users;

pub async fn health() -> &'static str { "ok" }

// todo: might have to move state into here
pub(crate) fn router_internal(state: SharedState) -> Router {
  Router::new()
  // fmt block
  .route("/health", routing::get(health))
  .nest("/users", users_router(state.clone()))
}
