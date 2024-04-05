use db::{
  models::{comment::Comment, item::Item},
  AuthToken, Timestamp, Title, Username,
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetEditItemResponse::default, example=GetEditItemResponse::default)]
pub struct GetEditItemResponse {
  pub id:               Uuid,
  pub username:         Username,
  pub title:            Title,
  /// news, show, ask
  pub item_type:        String,
  pub url:              Option<String>, // validate
  pub domain:           Option<String>,
  pub text:             Option<String>, // validate
  /// karma for the item
  pub points:           i32,
  /// internal algorithmic score to sort items on home page by popularity
  pub score:            i32, // todo: both points and score?
  pub comment_count:    i32,
  /// tweet, blog, paper, other
  pub item_category:    String, // validate
  pub created:          Timestamp,
  pub dead:             bool,
  /// unique to get-edit item page
  pub text_for_editing: Option<String>, // validate
}
impl From<Item> for GetEditItemResponse {
  fn from(item: Item) -> Self {
    // backlog(sanitize) - item text
    let text_for_editing = item.text.clone();
    Self {
      id: item.id,
      username: item.username,
      title: item.title,
      item_type: item.item_type,
      url: item.url,
      domain: item.domain,
      text: item.text,
      points: item.points,
      score: item.score,
      comment_count: item.comment_count,
      item_category: item.item_category,
      created: item.created,
      dead: item.dead,
      text_for_editing,
    }
  }
}
