// todo(cookie) - remove user cookie data - https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/index.js#L142

mod payload;
mod response;

pub use payload::*;
pub use response::*;

#[cfg(test)] mod test;

use anyhow::anyhow;
use axum::{
  debug_handler,
  extract::{Path, State},
  http::StatusCode,
  routing, Json, Router,
};
use db::{models::user::User, password::verify_user_password, AuthToken, DbError, Username};
use garde::Validate;
use payload::*;
use response::*;
use tracing::{debug, info, warn};

use super::SharedState;
use crate::{
  // auth::{self, assert_authenticated},
  error::ApiError,
  ApiResult,
};

// todo(auth)
fn todo_auth_token() -> AuthToken {
  AuthToken("temporaryytemporaryytemporaryytemporaryy".to_string())
}

pub fn users_router(state: SharedState) -> Router {
  Router::new()
    // note - called `/users/get-user-data` in reference
    .route("/:username", routing::get(get::get_user))
    .route("/", routing::put(put::update_user))
    .route("/:username", routing::delete(delete::delete_user))
    .route("/", routing::post(post::create_user))
    .route("/reset-password-link/:username", routing::put(put::request_password_reset_link))
    .route("/change-password", routing::put(put::change_password))
    .route("/login", routing::post(post::login))
    .route("/logout", routing::post(post::logout))
    .with_state(state)
}

pub(super) mod get {
  use super::*;

  #[utoipa::path(
      get,
      path = "/users/{username}",
      params( ("username" = String, Path, example = "alice") ),
      responses(
        // todo(auth) auth error
        // (status = 401, description = "Unauthorized"),
        (status = 422, description = "Invalid Payload"),
        (status = 422, description = "Invalid username"),
        (status = 500, description = "Database Error"),
        (status = 404, description = "User not found"),
        (status = 200, description = "Success", body = User),// todo(define reduced UserResponse body)
      ),
  )]
  /// Get user.
  ///
  /// If `username` exists, return the User. Otherwise, return NotFound.
  ///
  /// todo(auth): currently, we return the whole user. When auth is implemented, we will want to
  /// return different user data, per the caller's auth.
  ///
  /// ref get_public: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L223
  /// ref get_private: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L244
  pub async fn get_user(
    State(state): State<SharedState>,
    Path(username): Path<Username>,
    // auth_session: AuthSession,  // todo(auth)
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
pub(super) mod post {
  use axum::response::Redirect;

  use super::*;
  use crate::auth::{login_post_internal, logout_post_internal, AuthSession};

  #[utoipa::path(
      post,
      path = "/users",
      request_body = UserPayload,
      responses(
        (status = 422, description = "Invalid Payload"),
        (status = 409, description = "User Conflict"),
        (status = 500, description = "Database Error"),
        (status = 200, description = "Success", body = UserResponse),
      ),
  )]
  /// Create a new user:
  ///
  /// todo(search): tell the Algolia about the new user
  /// todo(session): spam prevention?
  /// todo(cookie) https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/index.js#L29
  pub async fn create_user(
    State(state): State<SharedState>,
    Json(payload): Json<UserPayload>,
  ) -> ApiResult<Json<UserResponse>> {
    debug!("create_user called with payload: {payload:?}");
    payload.validate(&())?;
    let user: User = {
      let mut user = payload.into_user().await;
      let auth_token = todo_auth_token();
      let expiration = crate::utils::default_expiration();
      user.auth_token = Some(auth_token);
      user.auth_token_expiration = Some(expiration);
      user
    };

    db::queries::users::create_user(&state.pool, &user).await?;

    let user_response = UserResponse::from(user);
    info!("created user: {user_response:?}");
    Ok(Json(user_response))
  }

  #[utoipa::path(
      post,
      path = "/users/login",
      request_body = CredentialsPayload,
      responses(
        // todo(testing): check documented routes
        (status = 422, description = "Invalid Payload"),
        (status = 409, description = "User Conflict"),
        (status = 500, description = "Database Error"),
        // todo: what to return?
        (status = 200, description = "Success", body = Redirect),
      ),
  )]
  /// User login.
  pub async fn login(
    mut auth_session: AuthSession,
    Json(payload): Json<CredentialsPayload>,
  ) -> ApiResult<StatusCode> {
    login_post_internal(auth_session, payload).await
  }

  #[utoipa::path(
      post,
      path = "/users/logout",
      responses(
        // todo(testing): check documented routes
        (status = 422, description = "Invalid Payload"),
        (status = 409, description = "User Conflict"),
        (status = 500, description = "Internal Server Error"),
        // todo: what to return
        (status = 200, description = "Success", body = Redirect),
      ),
  )]
  /// User logout.
  pub async fn logout(auth_session: AuthSession) -> ApiResult<StatusCode> {
    logout_post_internal(auth_session).await
  }

  // todo(auth): authenticate user: blocked
  // ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L97
  // todo(cookie): https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/index.js#L71
  // todo(cookie): https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/index.js#L124
}

pub(super) mod put {
  use super::*;

  #[utoipa::path(
      put,
      path = "/users",
      request_body = UserUpdatePayload,
      responses(
        // todo(auth) auth error
        // (status = 401, description = "Unauthorized"),
        (status = 422, description = "Invalid Payload"),
        (status = 500, description = "Database Error"),
        (status = 404, description = "User not found"),
        (status = 200, description = "Success"),
      ),
  )]
  /// Update the user's about or email field.
  ///
  /// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L287
  pub async fn update_user(
    State(state): State<SharedState>,
    // auth_session: AuthSession, // todo(auth)
    Json(payload): Json<UserUpdatePayload>,
  ) -> ApiResult<StatusCode> {
    debug!("update_user_about called with payload: {payload:?}");
    // assert_authenticated(&auth_session)?;
    payload.validate(&())?;
    if payload.about.is_none() && payload.email.is_none() {
      return Err(ApiError::MissingField("about or email must be provided".to_string()));
    }

    db::queries::users::update_user(&state.pool, &payload.username, &payload.about, &payload.email)
      .await?;

    info!("updated user about for: {}", payload.username);
    Ok(StatusCode::OK)
  }

  #[utoipa::path(
      put,
      path = "/users/reset-password-link/{username}",
      params( ("username" = String, Path, example = "alice") ),
      responses(
        // todo(auth) auth error
        // (status = 401, description = "Unauthorized", body = ApiError::Unauthorized),
        (status = 422, description = "Invalid username"),
        (status = 500, description = "Database Error"),
        (status = 404, description = "User not found"),
        (status = 404, description = "No email found"),
        (status = 200, description = "Success"),
      ),
  )]
  /// Request a password reset link.
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
    // todo(email): use the email api to send a reset password email
    // send_reset_email(&email, &reset_password_token).await?;

    info!("sent password reset email to: {email}");
    Ok(StatusCode::OK)
  }

  #[utoipa::path(
      put,
      path = "/users/change-password",
      request_body = ChangePasswordPayload,
      responses(
        // todo(auth) auth error
        // (status = 401, description = "Unauthorized", body = ApiError::Unauthorized),
        (status = 422, description = "Payload Validation Error"),
        (status = 500, description = "Database Error"),
        (status = 404, description = "User not found"),
        (status = 401, description = "Incorrect Password"),
        (status = 200, description = "Success"),
      ),
  )]
  /// Change user password.
  ///
  /// todo(cookie) ref - https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/index.js#L267
  pub async fn change_password(
    State(state): State<SharedState>,
    Json(payload): Json<ChangePasswordPayload>,
  ) -> ApiResult<StatusCode> {
    debug!("change_password called with payload: {payload:?}");
    payload.validate(&())?;
    let user = db::queries::users::get_user(&state.pool, &payload.username)
      .await?
      .ok_or(ApiError::DbEntryNotFound("no such user".to_string()))?;
    if !verify_user_password(&user, payload.current_password)? {
      return Err(ApiError::IncorrectPassword("incorrect password".to_string()));
    }

    let password_hash = db::password::hash_password_argon(&payload.new_password).await?;
    db::queries::users::update_user_password(&state.pool, &payload.username, &password_hash)
      .await?;
    // todo(email) - send an email to the user that their password has changed

    info!("changed password for user: {}", payload.username);
    Ok(StatusCode::OK)
  }
}

pub(super) mod delete {
  use super::*;

  #[utoipa::path(
      delete,
      path = "/users/{username}",
      params( ("username" = String, Path, example = "alice") ),
      responses(
        // todo(auth) auth error
        // (status = 401, description = "Unauthorized", body = ApiError::Unauthorized),
        (status = 422, description = "Invalid username"),
        (status = 500, description = "Database Error"),
        (status = 404, description = "User not found"),
        (status = 200, description = "Success"),
      ),
  )]
  /// Delete a user.
  pub async fn delete_user(
    State(state): State<SharedState>,
    Path(username): Path<Username>,
    // auth_session: AuthSession, // todo(auth), todo(mods)
  ) -> ApiResult<StatusCode> {
    debug!("delete_user called with username: {username}");
    // assert_authenticated(&auth_session)?;
    username.validate(&())?;
    db::queries::users::delete_user(&state.pool, &username).await?;

    info!("deleted user: {}", username);
    Ok(StatusCode::OK)
  }
}
