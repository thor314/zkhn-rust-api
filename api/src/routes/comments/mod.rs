mod payload;
mod response;
mod routes;
#[cfg(test)] mod test;

use axum::Router;
pub use payload::*;
pub use response::*;

use crate::SharedState;

// todo
pub fn comments_router(state: SharedState) -> Router { Router::new().with_state(state) }
