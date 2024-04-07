//! Provided methods:
//! - `create_comment`
//! - `get_comment`
//! - `update_comment_vote`
//! - `update_comment_favorite`
//! - `update_comment_text`
//! - `delete_comment`

use super::*;

// corresponding to `add_new_comment` in API
#[derive(Debug, Deserialize, Validate)]
pub struct CommentPayload {
  #[garde(dive)]
  pub username:          Username,
  #[garde(skip)]
  pub parent_item_id:    Uuid,
  #[garde(dive)]
  pub parent_item_title: Title,
  #[garde(skip)]
  pub is_parent:         bool,
  #[garde(skip)]
  pub root_comment_id:   Option<Uuid>,
  #[garde(skip)]
  pub parent_comment_id: Option<Uuid>,
  #[garde(dive)]
  pub text:              CommentText,
  #[garde(skip)]
  pub dead:              bool,
}

impl TryFrom<CommentPayload> for Comment {
  type Error = ApiError;

  fn try_from(payload: CommentPayload) -> ApiResult<Self> {
    payload.validate(&())?;

    let comment = Comment::new(
      payload.username,
      payload.parent_item_id,
      payload.parent_item_title,
      payload.is_parent,
      payload.root_comment_id,
      payload.parent_comment_id,
      payload.text,
      payload.dead,
    );

    Ok(comment)
  }
}

impl CommentPayload {
  pub fn new(
    username: &str,
    parent_item_id: &Uuid,
    parent_item_title: &str,
    is_parent: bool,
    parent_comment_id: Uuid,
    text: &str,
    dead: bool,
  ) -> Self {
    {
      Self {
        username:          todo!(),
        parent_item_id:    todo!(),
        parent_item_title: todo!(),
        is_parent:         todo!(),
        root_comment_id:   todo!(),
        parent_comment_id: todo!(),
        text:              todo!(),
        dead:              todo!(),
      }
    }
  }
}

impl CommentPayload {
  /// Assume Comment Payload has already been validated.
  fn into_comment(self) -> Comment {
    Comment::new(
      self.username,
      self.parent_item_id,
      self.parent_item_title,
      self.is_parent,
      self.root_comment_id,
      self.parent_comment_id,
      self.text,
      self.dead,
    )
  }
}

use std::sync::Arc;

use anyhow::Context;
use axum::{
  extract::{Path, State},
  http::StatusCode,
  Json, Router,
};
use axum_login::AuthUser;
use db::{
  models::{
    comment::{self, Comment},
    user_vote::{self, UserVote, VoteState},
  },
  queries, DbError,
};
use futures::{select, FutureExt};
use garde::Validate;
use tokio::spawn;
use uuid::Uuid;

use crate::{
  // auth::{self, assert_authenticated, AuthSession},
  error::ApiError,
  ApiResult,
  DbPool,
};
