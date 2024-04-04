use db::{
  models::{comment::Comment, item::Item},
  AuthToken, Timestamp, Username,
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::COMMENTS_PER_PAGE;
#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetItemResponse::default, example=GetItemResponse::default)]
pub struct GetItemResponse {
  pub item:             Item,
  pub comments:         Vec<Comment>,
  pub is_more_comments: bool,
}

impl GetItemResponse {
  pub fn new(item: Item, comments: Vec<Comment>, page: usize) -> Self {
    let is_more_comments = comments.len() > page * COMMENTS_PER_PAGE;
    Self { item, comments, is_more_comments }
  }
}
