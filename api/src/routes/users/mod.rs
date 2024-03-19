pub mod payload;

use axum::{
  debug_handler,
  extract::{Path, State},
  http::StatusCode,
  routing, Json, Router,
};
use db::models::user::User;
use payload::UserPayload;
use tracing::info;

use crate::{
  auth::{self, assert_authenticated},
  error::ApiError,
  ApiResult, AuthSession, SharedState,
};

pub fn users_router(state: SharedState) -> Router {
  Router::new()
    .route("/:username", routing::get(get::get_user))
    .route("/", routing::post(post::create_user))
    .route("/", routing::patch(patch::update_user))
    .route("/:username", routing::delete(delete::delete_user))
    .with_state(state)
}

pub mod get {
  use anyhow::anyhow;

  use super::*;

  /// If `username` exists, return the User. Otherwise, return NotFound.
  pub async fn get_user(
    State(state): State<SharedState>,
    Path(username): Path<String>,
    // auth_session: AuthSession, // keep commented to denote that no auth required
  ) -> ApiResult<Json<User>> {
    let pool = &state.pool;
    let user = db::queries::users::get_user(pool, &username)
      .await?
      .ok_or(ApiError::DbEntryNotFound("that user does not exist".to_string()))?;
    Ok(Json(user))
  }
}

// note to self that put is for updating, post is for creating. Puts should be idempotent.
pub mod post {
  use super::*;

  // todo: how to spam prevention?
  pub async fn create_user(
    State(state): State<SharedState>,
    // auth_session: AuthSession, // keep commented to denote that no auth required
    Json(user_payload): Json<UserPayload>,
  ) -> ApiResult<StatusCode> {
    let user: User = user_payload.try_into()?;
    db::queries::users::create_user(&state.pool, &user).await?;

    Ok(StatusCode::CREATED)
  }
}

pub mod patch {
  use self::payload::UserUpdatePayload;
  use super::*;

  // todo: this is a crap way to do an api, do it better, probably define an update payload or
  // something
  pub async fn update_user(
    State(state): State<SharedState>,
    // auth_session: AuthSession,
    Json(payload): Json<UserUpdatePayload>,
  ) -> ApiResult<StatusCode> {
    println!("username: {}", payload.username);
    // assert_authenticated(&auth_session)?;

    // todo: validate input
    db::queries::users::update_user_about(&state.pool, &payload.username, &payload.about.unwrap())
      .await?;
    Ok(StatusCode::OK)
  }
}

pub mod delete {
  use super::*;

  pub async fn delete_user(
    State(state): State<SharedState>,
    Path(username): Path<String>,
    // auth_session: AuthSession,
  ) -> ApiResult<StatusCode> {
    // assert_authenticated(&auth_session)?;
    db::queries::users::delete_user(&state.pool, &username).await?;
    Ok(StatusCode::OK)
  }
}
