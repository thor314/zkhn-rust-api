use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use diesel::{prelude::*, sql_types::*, QueryDsl, Queryable, Selectable, SelectableHelper};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid as Uid;

use crate::models::{
  user_favorite::UserFavorite,
  //   user_hidden::UserHidden,
  //   user_vote::{UserVote, VoteType},
};
use crate::{error::PasswordError, schema::users};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
// match to a schema for selectable
#[diesel(table_name = users)]
// use postgres, improve compiler error messages.
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub id: Uid,
  pub username: String,
  /// Hashed password.
  // todo: look for a password hash wrapper
  pub password: String,
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
      password,
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

  /// Hashes the user's password before saving if it is modified or new.
  pub async fn hash_password_before_save(&mut self) -> Result<(), bcrypt::BcryptError> {
    self.password = bcrypt::hash(&self.password, bcrypt::DEFAULT_COST)?;
    Ok(())
  }

  pub async fn compare_password(&self, pw: &str) -> Result<bool, PasswordError> {
    bcrypt::verify(pw, &self.password).map_err(PasswordError::from)
  }

  pub fn favorite(&self, item_type: String, item_id: Uid) -> UserFavorite {
    UserFavorite { username: self.username.clone(), item_type, item_id, date: crate::utils::now() }
  }

  // pub fn hide(&self, item_id: Uid, item_creation_date: DateTime<Utc>) -> UserHidden {
  //   UserHidden { username: self.username.clone(), item_id, date: Utc::now(), item_creation_date }
  // }

  // pub fn vote(
  //   &self,
  //   vote_type: VoteType,
  //   content_id: Uid,
  //   parent_item_id: Option<Uid>,
  //   upvote: bool,
  // ) -> UserVote {
  //   let downvote = !upvote;
  //   UserVote {
  //     username: self.username.clone(),
  //     vote_type,
  //     content_id,
  //     parent_item_id,
  //     upvote,
  //     downvote,
  //     date: crate::utils::now(),
  //   }
  // }
}
