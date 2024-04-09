use super::*;

/// A single post on the site.
/// Note that an item either has a url and domain, or text, but not both.
/// Comments on a post
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(example = Item::default, default = Item::default)]
#[serde(rename_all = "camelCase")]
pub struct Item {
  pub id:            Uuid,
  pub username:      Username,
  pub title:         Title,
  pub item_type:     ItemType,
  pub url:           Option<Url>,
  pub domain:        Option<Domain>,
  pub text:          Option<Text>,
  /// upvotes for the item
  pub points:        i32,
  /// internal algorithmic score to sort items on home page by popularity
  pub score:         i32,
  pub item_category: ItemCategory,
  pub created:       Timestamp,
  pub dead:          bool,
}

impl Default for Item {
  fn default() -> Self {
    Item {
      id:            Uuid::new_v4(),
      username:      Username::default(),
      title:         Title::default(),
      item_type:     ItemType::default(),
      // note that an item either has a url and domain, or text, but not both
      // but it's convenient to have both for testing and documentation purposes
      url:           Some(Url::default()),
      domain:        Some(Domain::default()),
      text:          Some(Text::default()),
      points:        1,
      score:         0,
      item_category: ItemCategory::default(),
      created:       now(),
      dead:          false,
    }
  }
}

impl Item {
  pub fn new(
    username: Username,
    title: Title,
    item_type: ItemType,
    text_or_url_content: TextOrUrl,
    item_category: ItemCategory,
  ) -> Self {
    let (url, domain, text) = text_or_url_content.url_domain_text();

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

  pub fn modification_expiration(&self) -> Timestamp {
    self.created + chrono::Duration::try_hours(1).unwrap()
  }
}

#[derive(Default, Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "item_category_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ItemCategory {
  Tweet,
  Blog,
  Paper,
  #[default]
  Other,
}
impl fmt::Display for ItemCategory {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ItemCategory::Tweet => write!(f, "tweet"),
      ItemCategory::Blog => write!(f, "blog"),
      ItemCategory::Paper => write!(f, "paper"),
      ItemCategory::Other => write!(f, "other"),
    }
  }
}

#[derive(Default, Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "item_type_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
  #[default]
  News,
  Show,
  Ask,
}
impl fmt::Display for ItemType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ItemType::Ask => write!(f, "ask"),
      ItemType::News => write!(f, "news"),
      ItemType::Show => write!(f, "show"),
    }
  }
}
