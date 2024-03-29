use axum::{routing, Json, Router};
use axum_login::AuthManagerLayer;
use db::DbPool;
use tower_sessions_sqlx_store::PostgresStore;
use tracing::debug;

use self::{comments::comments_router, openapi::docs_router, users::users_router};
use crate::{auth::MyAuthLayer, SharedState};

// pub mod so that payloads and responses can be accessed by integration tests
pub mod comments;
pub mod items;
pub mod openapi;
pub mod user_votes;
pub mod users;

async fn health() -> &'static str { "ok" }

// pub(crate) fn routes(pool: DbPool, auth_layer: MyAuthLayer) -> Router {
pub(crate) fn routes(pool: DbPool) -> Router {
  debug!("Initializing routes...");
  let state = SharedState::new(pool);

  Router::new()
    //// login protected routes go above the login route_layer
    // .route_layer(login_required!(AuthBackend, login_url = "/login"))
    //// unprotected routes (like "/login") go below the login route_layer
    .route("/health", routing::get(health))
    .nest("/docs", docs_router())
    .nest("/users", users_router(state.clone()))
  // .merge(auth_router()) // todo(auth)
  // .nest("/items", items_router(state.clone()))
  // .nest("/comments", comments_router(state.clone()))
  // .layer(auth_layer) // todo(auth): may not be required here
  // ..
}
