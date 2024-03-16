use axum::{
  extract::{Path, State},
  http::StatusCode,
  Json,
};
use db::models::user::User;

// use sqlx::types::Uuid;
use crate::{
  auth::{self, assert_authenticated},
  error::{ApiError, RouteError},
  users::payload::UserPayload,
  ApiResult, AuthSession, SharedState,
};

pub async fn get_user(
  State(state): State<SharedState>,
  Path(username): Path<String>,
  // auth_session: AuthSession, // keep commented to denote that no auth required
) -> ApiResult<Json<User>> {
  let pool = &state.pool;
  let user = db::queries::users::get_user(pool, &username).await?.ok_or(RouteError::NotFound)?;
  Ok(Json(user))
}

// todo: how to spam prevention?
pub async fn create_user(
  State(state): State<SharedState>,
  Json(user_payload): Json<UserPayload>,
  // auth_session: AuthSession, // keep commented to denote that no auth required
) -> ApiResult<()> {
  let user: User = user_payload.try_into()?;
  db::queries::users::create_user(&state.pool, &user).await?;

  Ok(())
}

// todo: this is a crap way to do an api, do it better, probably define an update payload or
// something
pub async fn update_user_about(
  State(state): State<SharedState>,
  Path(username): Path<String>,
  Json(about): Json<String>,
  auth_session: AuthSession,
) -> ApiResult<()> {
  assert_authenticated(&auth_session)?;

  // todo: validate input
  db::queries::users::update_user_about(&state.pool, &username, &about).await?;
  Ok(())
}

pub async fn delete_user(
  State(state): State<SharedState>,
  Path(username): Path<String>,
  auth_session: AuthSession,
) -> ApiResult<()> {
  assert_authenticated(&auth_session)?;
  db::queries::users::delete_user(&state.pool, &username).await?;
  Ok(())
}
