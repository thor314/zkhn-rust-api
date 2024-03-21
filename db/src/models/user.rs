use scrypt::{
  password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
  Scrypt,
};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
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

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
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

impl User {
  pub fn new(
    username: Username,
    password_hash: PasswordHash,
    email: Option<Email>,
    about: Option<About>,
  ) -> Self {
    User {
      username,
      password_hash,
      auth_token: None,
      auth_token_expiration: None,
      reset_password_token: None,
      reset_password_token_expiration: None,
      email,
      created: now(),
      karma: 1,
      about,
      show_dead: false,
      is_moderator: false,
      shadow_banned: false,
      banned: false,
    }
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
