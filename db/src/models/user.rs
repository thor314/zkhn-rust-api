use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use diesel::{prelude::*, sql_types::*, QueryDsl, Queryable, Selectable, SelectableHelper};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use scrypt::{
  password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
  Scrypt,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid as Uid;

use super::{
  user_favorite::UserFavorite,
  user_hidden::UserHidden,
  user_vote::{UserVote, VoteType},
};
use crate::{
  error::{MyError, PasswordError},
  schema::users::{self, dsl::users as users_dsl},
};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize, Clone)]
// match to a schema for selectable
#[diesel(table_name = users)]
// use postgres, improve compiler error messages.
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub id: Uid,
  pub username: String,
  /// Hashed password.
  // todo: look for a password hash wrapper, this should be a hash
  pub password_hash: String,
  // todo: auth
  /// Authentication token.
  pub auth_token: Option<String>,
  /// Expiration of auth token.
  pub auth_token_expiration: Option<i64>,
  /// Reset password token.
  pub reset_password_token: Option<String>,
  /// Expiration of reset password token.
  pub reset_password_token_expiration: Option<i64>,
  /// User email.
  // todo: email wrapper
  pub email: String,
  /// Account creation timestamp.
  pub created: NaiveDateTime,
  /// User karma score.
  pub karma: i32,
  /// User biography.
  pub about: Option<String>,
  /// Flag to show dead posts.
  pub show_dead: bool,
  /// Is user a moderator.
  pub is_moderator: bool,
  /// Is user shadow banned.
  pub shadow_banned: bool,
  /// Is user banned.
  pub banned: bool,
}

impl User {
  pub fn new(username: String, password: String, email: String, about: Option<String>) -> Self {
    User {
      id: Uid::new_v4(),
      username,
      password_hash: password,
      auth_token: None,
      auth_token_expiration: None,
      reset_password_token: None,
      reset_password_token_expiration: None,
      email,
      created: crate::utils::now(),
      karma: 1,
      about,
      show_dead: false,
      is_moderator: false,
      shadow_banned: false,
      banned: false,
    }
  }

  pub fn verify_password(&self, other_password: &str) -> Result<bool, PasswordError> {
    let parsed_hash = PasswordHash::new(&self.password_hash)?;
    match Scrypt.verify_password(other_password.as_bytes(), &parsed_hash) {
      Ok(_) => Ok(true),
      Err(_) => Ok(false),
    }
  }

  pub fn favorite(&self, item_type: String, item_id: Uid) -> UserFavorite {
    UserFavorite { username: self.username.clone(), item_type, item_id, date: crate::utils::now() }
  }

  pub fn hide(&self, item_id: Uid, item_creation_date: NaiveDateTime) -> UserHidden {
    UserHidden {
      username: self.username.clone(),
      item_id,
      date: crate::utils::now(),
      item_creation_date,
    }
  }

  pub fn vote(
    &self,
    vote_type: VoteType,
    content_id: Uid,
    parent_item_id: Option<Uid>,
    upvote: bool,
  ) -> UserVote {
    let downvote = !upvote;
    UserVote {
      username: self.username.clone(),
      vote_type,
      content_id,
      parent_item_id,
      upvote,
      downvote,
      date: crate::utils::now(),
    }
  }
}

// todo: move this somewhere else?
/// Hashes the user's password before saving if it is modified or new.
pub fn hash_password(password: &str) -> Result<String, PasswordError> {
  let salt = SaltString::generate(&mut OsRng);
  let pw_hash: PasswordHash = Scrypt.hash_password(password.as_bytes(), &salt)?;
  Ok(pw_hash.to_string())
}

pub async fn increment_karma(conn: &mut AsyncPgConnection, username: &str) -> Result<(), MyError> {
  diesel::update(users_dsl.filter(users::username.eq(username)))
    .set(users::karma.eq(users::karma + 1))
    .execute(conn)
    .await?;

  Ok(())
}
