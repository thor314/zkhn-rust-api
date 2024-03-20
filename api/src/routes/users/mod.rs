//! Routes:
//! - create-new-user
//! - login-user
//! - authenticate-user
//! - logout-user
//! - logout-user-clear-cookies
//! - reset-user-password
//! - get-user-profile-data
//! - update-user-profile-data
//! - change-user-password (todo: diff reset password?)

pub mod payload;

use anyhow::anyhow;
use axum::{
  debug_handler,
  extract::{Path, State},
  http::StatusCode,
  routing, Json, Router,
};
use db::{models::user::User, DbError, Username};
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
    .route("/", routing::put(put::update_user))
    .route("/:username", routing::delete(delete::delete_user))
    .route("/", routing::post(post::create_user))
    .route("/login", routing::post(post::login_user))
    .route("/logout", routing::post(post::logout_user))
    .with_state(state)
}

pub mod get {
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

  /// Create a new user:
  ///
  /// - validate and create a new user from the payload
  /// - attempt to insert the new user into the db
  ///   - if the user already exists, return a 409
  /// todo: tell the Algolia about the new user
  /// todo: spam prevention?
  pub async fn create_user(
    State(state): State<SharedState>,
    // auth_session: AuthSession, // keep commented to denote that no auth required
    Json(user_payload): Json<UserPayload>,
  ) -> ApiResult<StatusCode> {
    let user: User = user_payload.try_into()?;
    let result = db::queries::users::create_user(&state.pool, &user).await;

    if let Err(DbError::Recoverable(e)) = result {
      return Err(ApiError::DbEntryAlreadyExists("user already exists".to_string()));
    }
    result?;

    Ok(StatusCode::CREATED)
  }

  use self::payload::UserUpdatePayload;
  use super::*;
  /// Log the user in, verify their password, and return their auth session info:
  /// - If the user does not exist, return NotFound.
  /// - If the user exists, but the password is incorrect, return Unauthorized.
  /// - Otherwise, create the user auth session and provide the new user auth token.
  pub async fn login_user(
    State(state): State<SharedState>,
    // only need username and password
    Json(user_payload): Json<UserPayload>,
    // ) -> ApiResult<AuthSession> {
  ) -> ApiResult<()> {
    let UserPayload { username, password, .. } = user_payload;

    let user = db::queries::users::get_user(&state.pool, &username.0)
      .await?
      .ok_or(ApiError::DbEntryNotFound("user not found".to_string()))?;

    if !user.verify_password(&password.0)? {
      return Err(ApiError::Unauthorized("invalid password".to_string()));
    }

    // todo create auth session, renew the user token
    // return Ok(auth_session);
    todo!()
  }

  /// todo
  pub async fn logout_user(
    State(state): State<SharedState>,
    auth_session: AuthSession,
  ) -> ApiResult<()> {
    assert_authenticated(&auth_session)?;
    let user = auth_session.user.unwrap().0;
    // user.auth_token = None;
    // user.auth_token_expiration = None;
    // todo update the user in the db
    db::queries::logout_user(&state.pool, &Username(user.username)).await?;
    Ok(())
  }
}

mod put {
  use super::*;
  use crate::UserUpdatePayload;

  // todo: this is a crap way to do an api, do it better, probably define an update payload or
  // something
  pub async fn update_user(
    State(state): State<SharedState>,
    // auth_session: AuthSession,
    Json(payload): Json<UserUpdatePayload>,
  ) -> ApiResult<StatusCode> {
    println!("username: {:?}", payload.username);
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
