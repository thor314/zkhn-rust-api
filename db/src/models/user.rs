use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use utoipa::ToSchema;
use uuid::Uuid;

use super::{
  user_favorite::UserFavorite,
  user_hidden::UserHidden,
  user_vote::{UserVote, VoteState},
};
use crate::{
  error::DbError,
  utils::{self, now},
  About, AuthToken, DbPool, Email, Password, PasswordHash, ResetPasswordToken, Timestamp, Username,
};

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, ToSchema)]
#[schema(example = User::default, default = User::default)]
pub struct User {
  pub username: Username,
  /// Hashed password
  pub password_hash: PasswordHash,
  // todo: oauth
  /// Authentication token
  pub auth_token: Option<AuthToken>,
  /// Expiration of auth token
  pub auth_token_expiration: Option<Timestamp>,
  /// Reset password token
  pub reset_password_token: Option<ResetPasswordToken>,
  /// Expiration of reset password token
  pub reset_password_token_expiration: Option<Timestamp>,
  /// User email
  pub email: Option<Email>,
  /// Account creation timestamp
  pub created: Timestamp,
  /// User karma score
  pub karma: i32,
  /// User biography
  pub about: Option<About>,
  /// Flag to show dead posts
  pub show_dead: bool,
  /// Is user a moderator
  pub is_moderator: bool,
  /// Is user shadow banned
  pub shadow_banned: bool,
  /// Is user banned
  pub banned: bool,
}

impl Default for User {
  fn default() -> Self {
    User {
      username: Username("alice".to_string()),
      password_hash: PasswordHash("password".to_string()),
      auth_token: None,
      auth_token_expiration: None,
      reset_password_token: None,
      reset_password_token_expiration: None,
      email: None,
      created: now(),
      karma: 1,
      about: None,
      show_dead: false,
      is_moderator: false,
      shadow_banned: false,
      banned: false,
    }
  }
}

impl User {
  pub fn new(
    username: Username,
    password_hash: PasswordHash,
    email: Option<Email>,
    about: Option<About>,
  ) -> Self {
    User { username, password_hash, email, about, ..Default::default() }
  }

  // todo: probably move
  pub fn favorite(&self, item_type: String, item_id: Uuid) -> UserFavorite {
    UserFavorite { username: self.username.clone(), item_type, item_id, date: now() }
  }

  // todo: probably move
  pub fn hide(&self, item_id: Uuid, item_creation_date: Timestamp) -> UserHidden {
    UserHidden { username: self.username.clone(), item_id, date: now(), item_creation_date }
  }

  // todo: probably move
  pub fn vote(
    &self,
    vote_type: String,
    content_id: Uuid,
    parent_item_id: Option<Uuid>,
    vote_state: VoteState,
    upvote: bool,
  ) -> UserVote {
    let downvote = !upvote;
    UserVote {
      username: self.username.clone(),
      vote_type,
      content_id,
      parent_item_id,
      vote_state,
      created: now(),
    }
  }
}

// explicitly redact sensitive info
impl std::fmt::Debug for User {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("User")
      .field("username", &self.username)
      .field("password_hash", &self.password_hash)
      .field("auth_token", &"redacted")
      .field("auth_token_expiration", &self.auth_token_expiration)
      .field("reset_password_token", &"redacted")
      .field("reset_password_token_expiration", &self.reset_password_token_expiration)
      .field("email", &self.email)
      .field("created", &self.created)
      .field("karma", &self.karma)
      .field("about", &self.about)
      .field("show_dead", &self.show_dead)
      .field("is_moderator", &self.is_moderator)
      .field("shadow_banned", &self.shadow_banned)
      .field("banned", &self.banned)
      .finish()
  }
}
