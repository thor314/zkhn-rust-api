use axum::Router;

use self::users::users_router;
use crate::SharedState;

// pub mod items;
pub mod users;

// todo: might have to move state into here
pub(crate) fn router_internal(state: SharedState) -> Router {
  Router::new()
  // fmt block
  .nest("/users", users_router(state.clone()))
}
