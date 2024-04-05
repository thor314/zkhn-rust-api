use db::{
  models::{
    comment::Comment,
    item::{Item, ItemCategory, ItemType},
  },
  AuthToken, Domain, Text, Timestamp, Title, Url, Username,
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::COMMENTS_PER_PAGE;

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetItemResponse::default, example=GetItemResponse::default)]
#[serde(rename_all = "camelCase")]
pub struct GetItemResponse {
  pub item:             Item,
  pub comments:         Vec<Comment>, // todo: transform reduce comment
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
#[serde(rename_all = "camelCase")]
pub struct GetEditItemResponse {
  pub id:               Uuid,
  pub username:         Username,
  pub title:            Title,
  /// news, show, ask
  pub item_type:        ItemType,
  pub url:              Option<Url>,
  pub domain:           Option<Domain>,
  pub text:             Option<Text>,
  /// how many upvotes
  pub points:           i32,
  /// internal algorithmic score to sort items on home page by popularity
  pub score:            i32,
  /// tweet, blog, paper, other
  pub item_category:    ItemCategory,
  pub created:          Timestamp,
  pub dead:             bool,
  /// unique to get-edit item page
  pub text_for_editing: Option<Text>,
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
      item_category: item.item_category,
      created: item.created,
      dead: item.dead,
      text_for_editing,
    }
  }
}
