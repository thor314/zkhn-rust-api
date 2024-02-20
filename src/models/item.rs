use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
  types::{Text, Uuid},
  FromRow,
};

use super::comment::Comment;

/// A single post on the site.
/// Note that an item either has a url and domain, or text, but not both.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Item {
  pub id:            Uuid,
  pub by:            String,
  pub title:         String,
  /// news, show ask, etc.
  #[serde(rename = "type")]
  pub item_type:     String, // `type` is a reserved keyword in Rust
  pub url:           Option<String>,
  pub domain:        Option<String>,
  pub text:          Option<String>,
  /// karma for the item
  pub points:        i32,
  /// internal algorithmic score to sort items on home page by popularity
  pub score:         i32, // todo: both points and score?
  pub comment_count: u32,
  pub category:      ItemCategory,
  pub created:       DateTime<Utc>,
  pub dead:          bool,
}

impl Item {
  pub fn new(
    by: String,
    title: String,
    item_type: String,
    is_text: bool,
    text_or_url_content: String,
    category: ItemCategory,
  ) -> Self {
    let (url, domain, text) = if is_text {
      (None, None, Some(text_or_url_content.clone()))
    } else {
      let url = text_or_url_content.clone();
      let domain = url::Url::parse(&url).unwrap().domain().unwrap().to_string();
      (Some(url), Some(domain), None)
    };

    Item {
      id: Uuid::new_v4(),
      by,
      title,
      item_type,
      url,
      domain,
      text,
      points: 1,
      score: 0,
      comment_count: 0,
      category,
      created: Utc::now(),
      dead: false,
    }
  }

  pub fn create_comment(&self, by: String, text: String) -> Comment {
    Comment::new(by, self.id, self.title.clone(), true, None, None, text)
  }

  pub fn kill(&mut self) { self.dead = true; }

  pub fn unkill(&mut self) { self.dead = false; }
}

// todo: add other types rest
#[derive(Debug, Serialize, Deserialize)]
pub enum ItemCategory {
  Tweet,
  Blog,
  Paper,
  Other,
}
