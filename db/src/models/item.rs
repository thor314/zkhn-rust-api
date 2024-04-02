use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use utoipa::ToSchema;
use uuid::Uuid;

use super::comment::Comment;
use crate::{error::DbError, utils::now, Timestamp, Title, Username};

/// A single post on the site.
/// Note that an item either has a url and domain, or text, but not both.
/// Comments on a post
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(example = Item::default, default = Item::default)]
pub struct Item {
  pub id:            Uuid,
  pub username:      Username,
  pub title:         Title,
  /// news, show, ask
  pub item_type:     String,
  pub url:           Option<String>, // validate
  pub domain:        Option<String>,
  pub text:          Option<String>, // validate
  /// karma for the item
  pub points:        i32,
  /// internal algorithmic score to sort items on home page by popularity
  pub score:         i32, // todo: both points and score?
  pub comment_count: i32,
  /// Tweet, Blog, Paper, Other
  pub item_category: String, // validate
  pub created:       Timestamp,
  pub dead:          bool,
}

impl Default for Item {
  fn default() -> Self {
    Item {
      id:            Uuid::new_v4(),
      username:      Username::default(),
      title:         Title::default(),
      item_type:     "news".to_string(),
      url:           Some("https://example.com".to_string()),
      domain:        Some("example.com".to_string()),
      text:          None,
      points:        1,
      score:         0,
      comment_count: 0,
      item_category: "Tweet".to_string(),
      created:       now(),
      dead:          false,
    }
  }
}

impl Item {
  pub fn new(
    username: Username,
    title: Title,
    item_type: String,
    is_text: bool,
    text_or_url_content: String,
    item_category: String,
  ) -> Self {
    let (url, domain, text) = if is_text {
      (None, None, Some(text_or_url_content.clone()))
    } else {
      let url = text_or_url_content.clone();
      let domain = url::Url::parse(&url).unwrap().domain().unwrap().to_string();
      (Some(url), Some(domain), None)
    };

    Item { username, title, item_type, url, domain, text, item_category, ..Default::default() }
  }
}

// // todo: add other types rest
// #[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
// #[sqlx(type_name = "item_category_enum")]
// pub enum ItemCategory {
//   Tweet,
//   Blog,
//   Paper,
//   Other,
// }

// #[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
// #[sqlx(type_name = "item_type", rename_all = "lowercase")]
// // #[serde(rename_all = "lowercase")]
// pub enum ItemType {
//   News,
//   Show,
//   Ask,
// }
