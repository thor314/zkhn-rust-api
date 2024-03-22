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
use db::{models::user::User, DbError, Username};
use garde::Validate;
use payload::UserPayload;
use tracing::info;

use crate::{
  // auth::{self, assert_authenticated},
  error::ApiError,
  ApiResult,
  SharedState,
};

pub fn users_router(state: SharedState) -> Router {
  Router::new()
    .route("/:username", routing::get(get::get_user))
    .route("/", routing::put(put::update_user))
    .route("/:username", routing::delete(delete::delete_user))
    .route("/", routing::post(post::create_user))
    // .route("/login", routing::post(post::login_user))
    // .route("/logout", routing::post(post::logout_user))
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
    let username = Username(username);
    username.validate(&())?;
    let user = db::queries::users::get_user(pool, &username)
      .await?
      .ok_or(ApiError::DbEntryNotFound("that user does not exist".to_string()))?;
    Ok(Json(user))
  }
}

// note to self that put is for updating, post is for creating. Puts should be idempotent.
pub mod post {
  use axum::Form;
  use db::password::verify_user_password;

  use self::{payload::UserUpdatePayload, responses::UserResponse};
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
    WithValidation(user_payload): WithValidation<Json<UserPayload>>,
  ) -> ApiResult<Json<UserResponse>> {
    let user: User = {
      let mut user = user_payload.into_inner().into_user();
      // let (auth_token, auth_token_exp) = auth::temp_jank::generate_user_token();
      let (auth_token, auth_token_exp) = todo!();
      user.auth_token = Some(auth_token);
      user.auth_token_expiration = Some(auth_token_exp);
      user
    };
    let result = db::queries::users::create_user(&state.pool, &user).await;

    if let Err(DbError::Recoverable(e)) = result {
      tracing::error!("error creating user: {}", e);
      return Err(ApiError::DbEntryAlreadyExists("user already exists".to_string()));
    }
    result?;

    let user_response = UserResponse::from(user);
    info!("created user: {:?}", user_response);
    Ok(Json(user_response))
  }

  // /// Log the user in, verify their password, and return their auth session info:
  // /// - If the user does not exist, return NotFound.
  // /// - If the user exists, but the password is incorrect, return Unauthorized.
  // /// - Otherwise, create the user auth session and provide the new user auth token.
  // pub async fn login_user_password(
  //   mut auth_session: AuthSession,
  //   Form(creds): Form<PasswordCredentials>,
  // ) -> ApiResult<Json<UserResponse>> {
  //   let UserPayload { username, password, .. } = user_payload.into_inner();

  //   let user = db::queries::users::get_user(&state.pool, &username)
  //     .await?
  //     .ok_or(ApiError::DbEntryNotFound("user not found".to_string()))?;

  //   if !verify_user_password(&user, &password)? {
  //     tracing::error!("invalid password for user: {}", username.0);
  //     return Err(ApiError::PwError("invalid password".to_string()));
  //   }

  //   // renew user token: create a new unique string and store it
  //   let (auth_token, auth_token_expiration) = auth::temp_jank::generate_user_token();
  //   db::queries::store_user_auth_token(&state.pool, &username, &auth_token,
  // &auth_token_expiration)     .await?;
  //   let user_response = UserResponse::new(user, auth_token, auth_token_expiration);

  //   info!("logged in user: {}", username.0);
  //   Ok(Json(user_response))
  // }

  // pub async fn logout_user(
  //   State(state): State<SharedState>,
  //   auth_session: AuthSession,
  //   Path(token): Path<String>,
  // ) -> ApiResult<()> {
  //   assert_authenticated(&auth_session)?;
  //   let username = auth_session.user.unwrap().username;
  //   // user.auth_token = None;
  //   // user.auth_token_expiration = None;
  //   // todo update the user in the db
  //   db::queries::logout_user(&state.pool, &username.0).await?;

  //   info!("logged out user: {}", username);
  //   Ok(())
  // }
}

mod put {
  use super::*;
  use crate::UserUpdatePayload;

  // todo: this is a crap way to do an api, do it better, probably define an update payload or
  // something
  pub async fn update_user(
    State(state): State<SharedState>,
    // auth_session: AuthSession,
    WithValidation(payload): WithValidation<Json<UserUpdatePayload>>,
  ) -> ApiResult<StatusCode> {
    println!("username: {:?}", payload.username);
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
}

pub mod delete {
  use super::*;

  pub async fn delete_user(
    State(state): State<SharedState>,
    Path(username): Path<String>,
    // auth_session: AuthSession,
  ) -> ApiResult<StatusCode> {
    // assert_authenticated(&auth_session)?;
    let username = Username(username);
    username.validate(&())?;
    db::queries::users::delete_user(&state.pool, &username).await?;

    info!("deleted user: {}", username);
    Ok(StatusCode::OK)
  }
}
