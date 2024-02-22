use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use diesel::{prelude::*, sql_types::*, Queryable, Selectable};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid as Uid;

use crate::{
  error::MyError,
  models::comment::Comment,
  schema::{items, items::dsl::items as items_dsl},
};

/// A single post on the site.
/// Note that an item either has a url and domain, or text, but not both.
/// Comments on a post
#[derive(Queryable, Selectable, Debug)]
// match to a schema for selectable
#[diesel(table_name = items)]
// use postgres, improve compiler error messages.
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item {
  pub id:            Uid,
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
  pub created:       NaiveDateTime,
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
      id: Uid::new_v4(),
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
      created: crate::utils::now(),
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
#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::ItemCategoryEnum"]
pub enum ItemCategory {
  Tweet,
  Blog,
  Paper,
  Other,
}

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::ItemType"]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
  News,
  Show,
  Ask,
}

pub(crate) async fn increment_comments(
  conn: &mut AsyncPgConnection,
  parent_item_id: Uid,
) -> Result<(), MyError> {
  diesel::update(items_dsl.filter(items::id.eq(parent_item_id)))
    .set(items::comment_count.eq(items::comment_count + 1))
    .execute(conn)
    .await?;

  Ok(())
}
