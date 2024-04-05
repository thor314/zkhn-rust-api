// hack(cookie) - remove user cookie data - https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/index.js#L142

mod payload;
mod response;
use axum::{
  extract::{Path, State},
  http::StatusCode,
  routing, Json, Router,
};
use db::{models::user::User, queries::users, AuthToken, Username};
use garde::Validate;
use tracing::{debug, trace};

pub use self::{payload::*, response::*};
use super::SharedState;
use crate::{
  auth::{AuthSession, AuthenticationExt, PasswordExt},
  ApiError, ApiResult,
};

/// Router to be mounted at "/users"
pub(super) fn users_router(state: SharedState) -> Router {
  Router::new()
    // note - called `/users/get-user-data` in reference
    .route("/:username", routing::get(get::get_user))
    .route("/", routing::put(put::update_user).post(post::create_user))
    // todo(email) - create reset-password with reset password token
    .route("/reset-password-link/:username", routing::put(put::request_password_reset_link))
    .route("/change-password", routing::put(put::change_password))
    .route("/login", routing::post(post::login))
    .route("/logout", routing::post(post::logout))
    .route("/authenticate", routing::get(get::authenticate))
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
        (status = 200, body = GetUserResponse),
      ),
  )]
  /// Get user.
  ///
  /// If `username` exists, return the `UserResponse`. Otherwise, return NotFound.
  ///
  /// ref get_public: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L223
  /// ref get_private: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L244
  pub async fn get_user(
    State(state): State<SharedState>,
    Path(username): Path<Username>,
    auth_session: AuthSession,
  ) -> ApiResult<Json<GetUserResponse>> {
    trace!("get_user called with username: {username}");
    username.validate(&())?;
    let user = users::get_assert_user(&state.pool, &username).await?;
    let session_user = auth_session.get_user_from_session();
    let user_response = GetUserResponse::new(user, session_user);

    debug!("user response: {user_response:?}");
    Ok(Json(user_response))
  }

  #[utoipa::path(
      get,
      path = "/users/authenticate",
      responses(
        (status = 401, description = "Not logged in"),
        (status = 403, description = "Forbidden"),
        (status = 403, description = "Banned"),
        (status = 422, description = "Invalid username"),
        (status = 200, body = AuthenticateUserResponse),
      ),
  )]
  /// If the user is logged in as `username` and not banned, return information about the user.
  /// If the user is banned, or does not match username, return a 403.
  /// If the user is not logged in, return a 401.
  ///
  /// ref: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L97
  pub async fn authenticate(
    auth_session: AuthSession,
  ) -> ApiResult<Json<AuthenticateUserResponse>> {
    let session_user = auth_session.get_assert_user_from_session()?;
    let authenticate_user_response = AuthenticateUserResponse::new(session_user);
    debug!("authenticate_user_response: {authenticate_user_response:?}");
    Ok(Json(authenticate_user_response))
  }
}

pub(super) mod post {
  use super::*;
  use crate::auth::{login_post_internal, logout_post_internal};

  #[utoipa::path(
      post,
      path = "/users",
      request_body = CreateUserPayload,
      responses(
        (status = 422, description = "Invalid Payload"),
        (status = 409, description = "Duplication Conflict"),
        (status = 200),
      ),
  )]
  /// Create a new user.
  ///
  /// prod(search): tell the Algolia about the new user
  /// hack(cookie) https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/index.js#L29
  pub async fn create_user(
    State(state): State<SharedState>,
    Json(payload): Json<CreateUserPayload>,
  ) -> ApiResult<StatusCode> {
    trace!("create_user called with payload: {payload:?}");
    payload.validate(&())?;
    let user: User = payload.into_user().await;
    users::create_user(&state.pool, &user).await?;

    debug!("created user: {user:?}");
    Ok(StatusCode::OK)
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

  // hack(cookie): https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/index.js#L71
  // hack(cookie): https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/index.js#L124
}

pub(super) mod put {
  use db::{ResetPasswordToken, Timestamp};

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
    trace!("update_user_about called with payload: {payload:?}");
    let session_user = auth_session.get_assert_user_from_session()?;
    payload.validate(&())?;
    if payload.about.is_none() && payload.email.is_none() {
      return Err(ApiError::BadRequest("about or email must be provided".to_string()));
    }

    users::update_user(
      &state.pool,
      &session_user.username,
      &payload.about,
      &payload.email,
      &payload.show_dead,
    )
    .await?;

    debug!("updated user about for: {}", session_user.username);
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
  ///
  /// todo(password reset) - need to write a change password endpoint allowing user to use the token
  ///
  /// don't authorize for this route, user may have forgotten their password
  pub async fn request_password_reset_link(
    State(state): State<SharedState>,
    Path(username): Path<Username>,
  ) -> ApiResult<StatusCode> {
    trace!("request-password-reset-link called with username: {:?}", username);
    username.validate(&())?;
    let user = users::get_assert_user(&state.pool, &username).await?;
    let email = user.email.ok_or(ApiError::BadRequest("email missing".to_string()))?;

    // Generate a reset password token and expiration date for the user. Update the db.
    // prod(email)
    let reset_password_token = ResetPasswordToken("create reset password token".into());
    let reset_password_token_expiration = Timestamp::default_token_expiration();
    users::update_user_password_token(
      &state.pool,
      &username,
      &reset_password_token,
      &reset_password_token_expiration,
    )
    .await?;

    // blocked: mailgun-email-feature
    // prod(email): use the email api to send a reset password email
    // send_reset_email(&email, &reset_password_token).await?;

    debug!("sent password reset email to: {email}");
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
  /// Change user password. Do not require the user to be logged in.
  ///
  /// The user may either submit their current password, or a PasswordResetToken to identify
  /// themselves.
  ///
  /// hack(cookie) ref - https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/index.js#L267
  pub async fn change_password(
    State(state): State<SharedState>,
    Json(payload): Json<ChangePasswordPayload>,
  ) -> ApiResult<StatusCode> {
    trace!("change_password called with payload: {payload:?}");
    payload.validate(&())?;
    let user = users::get_assert_user(&state.pool, &payload.username).await?;

    if let Some(password) = payload.current_password {
      // user has submitted their old password as verification
      password.hash_and_verify(&user.password_hash).await?;
    } else if let Some(token) = payload.reset_password_token {
      // user has submitted a password reset token as verification
      let user = users::get_assert_user(&state.pool, &payload.username).await?;
      if user.reset_password_token.is_none() {
        return Err(ApiError::BadRequest("no reset password token found for user".to_string()));
      } else if user.reset_password_token.unwrap() != token {
        return Err(ApiError::UnauthorizedIncorrectToken);
      }
    } else {
      return Err(ApiError::BadRequest(
        "current_password or reset_password_token must be provided".to_string(),
      ));
    }

    users::update_user_password(&state.pool, &payload.username, &user.password_hash).await?;
    // prod(email) - send an email to the user that their password has changed

    debug!("changed password for user: {}", payload.username);
    Ok(StatusCode::OK)
  }
}
