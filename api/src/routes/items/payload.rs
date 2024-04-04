use db::{
  models::item::{self, Item},
  Title, Username,
};
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::ApiResult;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[schema(default = ItemPayload::default, example=ItemPayload::default)]
pub struct ItemPayload {
  #[garde(dive)]
  pub title:           Title,
  #[garde(skip)] // todo(itemtype)
  item_type: String,
  #[garde(skip)]
  is_text:             bool,
  // todo: could turn this to an enum
  #[garde(skip)] // todo(validate)
  text_or_url_content: String,
  #[garde(skip)] // todo(validate)
  item_category: String,
}

impl Default for ItemPayload {
  fn default() -> Self {
    Self {
      title:               Title::default(),
      item_type:           "news".into(),
      is_text:             true,
      text_or_url_content: "text content".into(),
      item_category:       "tweet".into(),
    }
  }
}

impl ItemPayload {
  pub async fn into_item(self, username: Username) -> Item {
    Item::new(
      username,
      self.title,
      self.item_type,
      self.is_text,
      self.text_or_url_content,
      self.item_category,
    )
  }

  /// convenience method for testing
  pub fn new(
    title: &str,
    item_type: &str,
    is_text: bool,
    text_or_url_content: &str,
    item_category: &str,
  ) -> ApiResult<Self> {
    let title = title.into();
    let item_type = item_type.into();
    let text_or_url_content = text_or_url_content.into();
    let item_category = item_category.into();

    let item_payload = Self { title, item_type, is_text, text_or_url_content, item_category };
    item_payload.validate(&())?;
    Ok(item_payload)
  }
}
