// todo(cookie) - remove user cookie data - https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/index.js#L142

mod payload;
mod response;
#[cfg(test)] mod test;

use axum::{
  extract::{Path, State},
  http::StatusCode,
  routing, Json, Router,
};
use db::{models::user::User, password::verify_user_password, AuthToken, Username};
use garde::Validate;
use tracing::{debug, info};

pub use self::{payload::*, response::*};
use super::SharedState;
use crate::{
  auth::{AuthSession, AuthenticationExt},
  error::ApiError,
  ApiResult,
};

/// Router to be mounted at "/users"
pub fn users_router(state: SharedState) -> Router {
  Router::new()
    // note - called `/users/get-user-data` in reference
    .route("/:username", routing::get(get::get_user).delete(delete::delete_user))
    .route("/", routing::put(put::update_user).post(post::create_user))
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
        (status = 422, description = "Invalid username"),
        (status = 404, description = "User not found"),
        (status = 200, body = User), // todo(define reduced UserResponse body)
      ),
  )]
  /// Get user.
  ///
  /// If `username` exists, return the User. Otherwise, return NotFound.
  ///
  /// ref get_public: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L223
  /// ref get_private: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L244
  pub async fn get_user(
    State(state): State<SharedState>,
    Path(username): Path<Username>,
    auth_session: AuthSession,
  ) -> ApiResult<Json<User>> {
    debug!("get_user called with username: {username}");
    // / todo(auth): currently, we return the whole user. When auth is implemented, we will want to
    // / return different user data, per the caller's auth.
    if auth_session.is_authenticated() {
      // todo(auth) - return reduced user data
      // let user = db::queries::users::get_user(&state.pool, &username).await?;
      // let user = user.map(|user| UserResponse::from(user));
      // return Ok(Json(user));
    } else {
      // todo
    }
    let pool = &state.pool;
    username.validate(&())?;
    let user = db::queries::users::get_user(pool, &username).await?;

    info!("found user: {user:?}");
    Ok(Json(user))
  }
}

pub(super) mod post {
  use super::*;
  use crate::auth::{login_post_internal, logout_post_internal};

  #[utoipa::path(
      post,
      path = "/users",
      request_body = UserPayload,
      responses(
        (status = 422, description = "Invalid Payload"),
        (status = 409, description = "Duplication Conflict"),
        (status = 200, body = UserResponse),
      ),
  )]
  /// Create a new user:
  ///
  /// todo(search): tell the Algolia about the new user
  /// todo(cookie) https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/index.js#L29
  pub async fn create_user(
    State(state): State<SharedState>,
    Json(payload): Json<UserPayload>,
  ) -> ApiResult<Json<UserResponse>> {
    debug!("create_user called with payload: {payload:?}");
    payload.validate(&())?;
    let user: User = payload.into_user().await;

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
        (status = 422, description = "Invalid Payload"),
        (status = 401, description = "Unauthorized: Incorrect Password"),
        (status = 200),
      ),
  )]
  /// User login.
  pub async fn login(
    auth_session: AuthSession,
    Json(payload): Json<CredentialsPayload>,
  ) -> ApiResult<StatusCode> {
    payload.validate(&())?;
    login_post_internal(auth_session, payload).await
  }

  #[utoipa::path(
      post,
      path = "/users/logout",
      responses(
        (status = 200),
      ),
  )]
  /// User logout.
  pub async fn logout(auth_session: AuthSession) -> ApiResult<StatusCode> {
    logout_post_internal(auth_session).await
  }

  // todo(auth): authenticate user:
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
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 400, description = "Bad Request"),
        (status = 422, description = "Invalid Payload"),
        (status = 404, description = "User not found"),
        (status = 200),
      ),
  )]
  /// Update the user's about or email field.
  ///
  /// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L287
  pub async fn update_user(
    State(state): State<SharedState>,
    auth_session: AuthSession,
    Json(payload): Json<UserUpdatePayload>,
  ) -> ApiResult<StatusCode> {
    debug!("update_user_about called with payload: {payload:?}");
    auth_session.caller_matches_payload(&payload.username)?;
    payload.validate(&())?;
    if payload.about.is_none() && payload.email.is_none() {
      return Err(ApiError::BadRequest("about or email must be provided".to_string()));
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
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Invalid username"),
        (status = 404, description = "User not found"),
        (status = 404, description = "No email stored for user"),
        (status = 200),
      ),
  )]
  /// Request a password reset link.
  pub async fn request_password_reset_link(
    State(state): State<SharedState>,
    Path(username): Path<Username>,
    auth_session: AuthSession,
  ) -> ApiResult<StatusCode> {
    debug!("request-password-reset-link called with username: {:?}", username);
    auth_session.caller_matches_payload(&username)?;
    username.validate(&())?;
    let user = db::queries::users::get_user(&state.pool, &username).await?;
    let email = user.email.ok_or(ApiError::BadRequest("email missing".to_string()))?;

    // Generate a reset password token and expiration date for the user. Update the db.
    // todo(email)
    let reset_password_token = AuthToken("create reset password token".into());
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
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Payload Validation Error"),
        (status = 404, description = "User not found"),
        (status = 200),
      ),
  )]
  /// Change user password.
  ///
  /// todo(cookie) ref - https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/index.js#L267
  pub async fn change_password(
    State(state): State<SharedState>,
    auth_session: AuthSession,
    Json(payload): Json<ChangePasswordPayload>,
  ) -> ApiResult<StatusCode> {
    debug!("change_password called with payload: {payload:?}");
    payload.validate(&())?;
    auth_session.caller_matches_payload(&payload.username)?;
    let user = db::queries::users::get_user(&state.pool, &payload.username).await?;
    // todo(password) - refactor to propagate error
    if !verify_user_password(&user, payload.current_password) {
      return Err(ApiError::Unauthorized("incorrect password".to_string()));
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
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Invalid username"),
        (status = 404, description = "User not found"),
        (status = 200),
      ),
  )]
  /// Delete a user.
  pub async fn delete_user(
    State(state): State<SharedState>,
    Path(username): Path<Username>,
    auth_session: AuthSession, // todo(mods)
  ) -> ApiResult<StatusCode> {
    debug!("delete_user called with username: {username}");
    auth_session.caller_matches_payload(&username)?;
    username.validate(&())?;
    db::queries::users::delete_user(&state.pool, &username).await?;

    info!("deleted user: {}", username);
    Ok(StatusCode::OK)
  }
}
