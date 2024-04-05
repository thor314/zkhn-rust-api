use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{utils::now, DbError, DbPool, DbResult, Domain, Text, Timestamp, Title, Url, Username};

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
  pub url:           Option<Url>,
  pub domain:        Option<Domain>,
  pub text:          Option<Text>,
  /// karma for the item
  pub points:        i32,
  /// internal algorithmic score to sort items on home page by popularity
  pub score:         i32, 
  /// tweet, blog, paper, other
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
      url:           Some(Url::default()),
      domain:        Some(Domain::default()),
      text:          Some(Text::default()),
      points:        1,
      score:         0,
      item_category: "tweet".to_string(),
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
      (None, None, Some(Text(text_or_url_content)))
    } else {
      let url = Url(text_or_url_content.clone());
      let domain: Domain = url.clone().into();
      (Some(url), Some(domain), None)
    };

    Item { username, title, item_type, url, domain, text, item_category, ..Default::default() }
  }

  /// An item is editable if it was created less than 1 hour ago, and has no comments.
  pub async fn assert_editable(&self, pool: &DbPool) -> DbResult<()> {
    if crate::queries::items::item_has_comments(pool, self.id).await {
      return Err(DbError::NotEditable("has comments".into()));
    } else if now() > self.modification_expiration() {
      return Err(DbError::NotEditable("expired".into()));
    }
    Ok(())
  }

  pub fn modification_expiration(&self) -> Timestamp { self.created + chrono::Duration::hours(1) }
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
