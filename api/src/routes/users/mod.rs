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
pub mod responses;

use anyhow::anyhow;
use axum::{
  debug_handler,
  extract::{Path, State},
  http::StatusCode,
  routing, Json, Router,
};
use axum_garde::WithValidation;
use db::{models::user::User, AuthToken, DbError, Username};
use garde::Validate;
use payload::UserPayload;
use tracing::{debug, info};

use crate::{
  // auth::{self, assert_authenticated},
  error::ApiError,
  ApiResult,
  SharedState,
};

fn todo_auth_token() -> AuthToken {
  AuthToken("temporaryytemporaryytemporaryytemporaryy".to_string())
}

pub fn users_router(state: SharedState) -> Router {
  Router::new()
    .route("/:username", routing::get(get::get_user))
    .route("/", routing::put(put::update_user))
    .route("/:username", routing::delete(delete::delete_user))
    .route("/", routing::post(post::create_user))
    .route("/users/reset_password_link/:username", routing::put(put::request_password_reset_link))
    .route("/users/change_password", routing::put(put::change_password))
    // .route("/login", routing::post(post::login_user))
    // .route("/logout", routing::post(post::logout_user))
    .with_state(state)
}

pub mod get {
  use super::*;

  /// If `username` exists, return the User. Otherwise, return NotFound.
  // todo(auth): currently, we return the whole user. When auth is implemented, we will want to
  // return different user data, per the caller's auth.
  //
  // ref get_public: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L223
  // ref get_private: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L244
  pub async fn get_user(
    State(state): State<SharedState>,
    Path(username): Path<Username>,
    // auth_session: AuthSession,  // todo
  ) -> ApiResult<Json<User>> {
    debug!("get_user called with username: {username}");
    let pool = &state.pool;
    username.validate(&())?;
    let user = db::queries::users::get_user(pool, &username)
      .await?
      .ok_or(ApiError::DbEntryNotFound("that user does not exist".to_string()))?;
    // todo(auth): currently, we return the whole user.
    // When auth is implemented, we will want to return different user data, per the caller's auth.
    info!("found user: {user:?}");
    Ok(Json(user))
  }
}

// note to self that put is for updating, post is for creating. Puts should be idempotent.
pub mod post {
  use db::{password::verify_user_password, AuthToken, RecoverableDbError};

  use self::{payload::UserUpdatePayload, responses::UserResponse};
  use super::*;
  use crate::PasswordCreds;

  /// Create a new user:
  ///
  /// - validate and create a new user from the payload
  /// - attempt to insert the new user into the db
  ///   - if the user already exists, return a 409.
  ///
  /// No authentication required.
  // todo: tell the Algolia about the new user
  // todo: spam prevention?
  pub async fn create_user(
    State(state): State<SharedState>,
    WithValidation(payload): WithValidation<Json<UserPayload>>,
  ) -> ApiResult<Json<UserResponse>> {
    debug!("create_user called with payload: {payload:?}");
    let user: User = {
      let mut user = payload.into_inner().into_user();
      let auth_token = todo_auth_token();
      let expiration = crate::utils::default_expiration();
      user.auth_token = Some(auth_token);
      user.auth_token_expiration = Some(expiration);
      user
    };
    let result = db::queries::users::create_user(&state.pool, &user).await;

    match result {
      Err(DbError::Recoverable(e)) => {
        match e {
          RecoverableDbError::DbEntryAlreadyExists => {
            tracing::warn!("duplicate user creation attempt {}", e);
            // todo: what error to return? Conflict?
          },
        };
      },
      Err(e) => return Err(ApiError::from(e)),
      Ok(_) => (),
    }

    let user_response = UserResponse::from(user);
    info!("created user: {user_response:?}");
    Ok(Json(user_response))
  }

  // login and logout - see auth
  // todo: authenticate user: blocked
  // ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L97
  // BLOCKED: https://github.com/maxcountryman/axum-login/pull/210
}

mod put {
  use db::password::verify_user_password;

  use super::*;
  use crate::{ChangePasswordPayload, UserUpdatePayload};

  // todo: this is a crap way to do an api, do it better, probably define an update payload or
  // something
  // ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L287
  pub async fn update_user(
    State(state): State<SharedState>,
    // auth_session: AuthSession, // todo
    WithValidation(payload): WithValidation<Json<UserUpdatePayload>>,
  ) -> ApiResult<StatusCode> {
    debug!("update_user called with payload: {payload:?}");
    // assert_authenticated(&auth_session)?;
    let payload = payload.into_inner();
    db::queries::users::update_user_about(
      &state.pool,
      &payload.username.0,
      &payload.about.map(|s| s.0).unwrap(),
    )
    .await?;

    info!("updated user: {}", payload.username);
    Ok(StatusCode::OK)
  }

  pub async fn request_password_reset_link(
    State(state): State<SharedState>,
    Path(username): Path<Username>,
  ) -> ApiResult<StatusCode> {
    debug!("request_password_reset_link called with username: {:?}", username);
    username.validate(&())?;
    let user = db::queries::users::get_user(&state.pool, &username)
      .await?
      .ok_or(ApiError::DbEntryNotFound("no such user".to_string()))?;
    let email = user.email.ok_or(ApiError::MissingField("email missing".to_string()))?;

    // Generate a reset password token and expiration date for the user. Update the db.
    let reset_password_token = todo_auth_token();
    let reset_password_token_expiration = crate::utils::default_expiration();
    db::queries::users::update_user_password_token(
      &state.pool,
      &username,
      &reset_password_token,
      &reset_password_token_expiration,
    )
    .await?;

    // blocked: mailgun-email-feature
    // todo: use the email api to send a reset password email
    // send_reset_email(&email, &reset_password_token).await?;

    info!("sent password reset email to: {email}");
    Ok(StatusCode::OK)
  }

  pub async fn change_password(
    State(state): State<SharedState>,
    WithValidation(payload): WithValidation<Json<ChangePasswordPayload>>,
  ) -> ApiResult<StatusCode> {
    debug!("change_password called with payload: {payload:?}");
    let user = db::queries::users::get_user(&state.pool, &payload.username)
      .await?
      .ok_or(ApiError::DbEntryNotFound("no such user".to_string()))?;
    if !verify_user_password(&user, &payload.current_password)? {
      return Err(ApiError::Unauthorized("incorrect password".to_string()));
    }

    let password_hash = db::password::hash_password(&payload.new_password)?;
    db::queries::users::update_user_password(&state.pool, &payload.username, &password_hash)
      .await?;
    // todo(email) - send an email to the user that their password has changed

    // todo(insecure) redact password after testing
    info!("changed password for user: {} to {:?}", payload.username, payload.new_password);
    Ok(StatusCode::OK)
  }
}

pub mod delete {
  use super::*;

  pub async fn delete_user(
    State(state): State<SharedState>,
    Path(username): Path<Username>,
    // auth_session: AuthSession,
  ) -> ApiResult<StatusCode> {
    debug!("delete_user called with username: {username}");
    // assert_authenticated(&auth_session)?;
    username.validate(&())?;
    db::queries::users::delete_user(&state.pool, &username).await?;

    info!("deleted user: {}", username);
    Ok(StatusCode::OK)
  }
}
