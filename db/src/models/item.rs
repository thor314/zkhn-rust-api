use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;

use super::comment::Comment;
use crate::{error::DbError, utils::{now, Timestamp}};

/// A single post on the site.
/// Note that an item either has a url and domain, or text, but not both.
/// Comments on a post
#[derive(sqlx::FromRow, Debug)]
pub struct Item {
  pub id:            Uuid,
  pub by:            String,
  pub title:         String,
  /// news, show ask, etc.
  pub item_type:     ItemType,
  pub url:           Option<String>,
  pub domain:        Option<String>,
  pub text:          Option<String>,
  /// karma for the item
  pub points:        i32,
  /// internal algorithmic score to sort items on home page by popularity
  pub score:         i32, // todo: both points and score?
  pub comment_count: i32,
  pub item_category: ItemCategory,
  pub created:       Timestamp,
  pub dead:          bool,
}

impl Item {
  pub fn new(
    by: String,
    title: String,
    item_type: ItemType,
    is_text: bool,
    text_or_url_content: String,
    item_category: ItemCategory,
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
      item_category,
      created: now(),
      dead: false,
    }
  }

  pub fn create_comment(&self, by: String, text: String, dead: bool) -> Comment {
    Comment::new(by, self.id, self.title.clone(), true, None, None, text, dead)
  }

  pub fn kill(&mut self) { self.dead = true; }

  pub fn unkill(&mut self) { self.dead = false; }
}

// todo: add other types rest
#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "item_category_enum")]
pub enum ItemCategory {
  Tweet,
  Blog,
  Paper,
  Other,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "item_type")]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
  News,
  Show,
  Ask,
}

pub(crate) async fn increment_comments(
  conn: &mut PgConnection,
  parent_item_id: Uuid,
) -> Result<(), DbError> {
  let query = r#"
    UPDATE items
    SET comment_count = comment_count + 1
    WHERE id = $1
  "#;
  sqlx::query(query).bind(parent_item_id).execute(conn).await?;

  Ok(())
}
