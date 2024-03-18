pub mod payload;

use axum::{
  debug_handler,
  extract::{Path, State},
  http::StatusCode,
  routing, Json, Router,
};
use db::models::user::User;
use payload::UserPayload;

use crate::{
  auth::{self, assert_authenticated},
  error::{ApiError, RouteError},
  ApiResult, AuthSession, SharedState,
};

pub fn users_router(state: SharedState) -> Router {
  Router::new()
    .route("/username", routing::get(get::get_user))
    .route("/", routing::put(put::create_user))
    .route("/username/about", routing::post(post::update_user_about))
    .route("/username", routing::delete(delete::delete_user))
    .with_state(state)
}

pub mod get {
  use super::*;

  pub async fn get_user(
    State(state): State<SharedState>,
    Path(username): Path<String>,
    // auth_session: AuthSession, // keep commented to denote that no auth required
  ) -> ApiResult<Json<User>> {
    let pool = &state.pool;
    let user = db::queries::users::get_user(pool, &username).await?.ok_or(RouteError::NotFound)?;
    Ok(Json(user))
  }
}

pub mod put {
  use super::*;

  // todo: how to spam prevention?
  pub async fn create_user(
    State(state): State<SharedState>,
    // auth_session: AuthSession, // keep commented to denote that no auth required
    Json(user_payload): Json<UserPayload>,
  ) -> ApiResult<()> {
    let user: User = user_payload.try_into()?;
    db::queries::users::create_user(&state.pool, &user).await?;

    Ok(())
  }
}

pub mod post {
  use super::*;

  // todo: this is a crap way to do an api, do it better, probably define an update payload or
  // something
  pub async fn update_user_about(
    State(state): State<SharedState>,
    Path(username): Path<String>,
    auth_session: AuthSession,
    Json(about): Json<String>,
  ) -> ApiResult<()> {
    assert_authenticated(&auth_session)?;

    // todo: validate input
    db::queries::users::update_user_about(&state.pool, &username, &about).await?;
    Ok(())
  }
}

pub mod delete {
  use super::*;

  pub async fn delete_user(
    State(state): State<SharedState>,
    Path(username): Path<String>,
    auth_session: AuthSession,
  ) -> ApiResult<()> {
    assert_authenticated(&auth_session)?;
    db::queries::users::delete_user(&state.pool, &username).await?;
    Ok(())
  }
}
