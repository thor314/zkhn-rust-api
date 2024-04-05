use db::{
  models::item::{self, Item, ItemCategory, ItemType},
  Text, Title, Url, Username,
};
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::ApiResult;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(default = CreateItemPayload::default, example=CreateItemPayload::default)]
pub struct CreateItemPayload {
  #[garde(dive)]
  pub title:           Title,
  #[garde(skip)]
  item_type:           ItemType,
  #[garde(skip)]
  is_text:             bool,
  // #[garde(dive)] // todo(validate)
  //  text_or_url_content: TextOrUrl,
  #[garde(skip)] // todo(validate)
  text_or_url_content: String,
  #[garde(skip)]
  item_category:       ItemCategory,
}

impl Default for CreateItemPayload {
  fn default() -> Self {
    Self {
      title:               Title::default(),
      item_type:           ItemType::default(),
      is_text:             true,
      text_or_url_content: "text content".into(),
      item_category:       ItemCategory::default(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
// #[schema(default = CreateItemPayload::default, example=CreateItemPayload::default)]
pub enum TextOrUrl {
  Text(#[garde(dive)] Text),
  Url(#[garde(dive)] Url),
}

impl CreateItemPayload {
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
    item_type: ItemType,
    is_text: bool,
    text_or_url_content: &str,
    item_category: ItemCategory,
  ) -> ApiResult<Self> {
    let title = title.into();
    let text_or_url_content = text_or_url_content.into();

    let item_payload = Self { title, item_type, is_text, text_or_url_content, item_category };
    item_payload.validate(&())?;
    Ok(item_payload)
  }
}

/// A payload for voting on an item or comment
#[derive(Default, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(default = VotePayload::default, example=VotePayload::default)]
#[serde(rename_all = "camelCase")]
pub struct VotePayload {
  pub id:   Uuid,
  pub vote: VotePayloadEnum,
}
impl VotePayload {
  pub fn new(id: Uuid, vote: VotePayloadEnum) -> Self { Self { id, vote } }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum VotePayloadEnum {
  Upvote,
  Downvote,
  Unvote,
}
impl Default for VotePayloadEnum {
  fn default() -> Self { Self::Upvote }
}

/// A payload for favoriting on an item or comment
#[derive(Default, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(default = FavoritePayload::default, example=FavoritePayload::default)]
#[serde(rename_all = "camelCase")]
pub struct FavoritePayload {
  pub id:       Uuid,
  pub favorite: FavoritePayloadEnum,
}
impl FavoritePayload {
  pub fn new(id: Uuid, favorite: FavoritePayloadEnum) -> Self { Self { id, favorite } }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum FavoritePayloadEnum {
  Favorite,
  Unfavorite,
}
impl Default for FavoritePayloadEnum {
  fn default() -> Self { Self::Favorite }
}

/// A payload for hiding an item or comment
#[derive(Default, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(default = HiddenPayload::default, example=HiddenPayload::default)]
#[serde(rename_all = "camelCase")]
pub struct HiddenPayload {
  pub id:     Uuid,
  pub hidden: HiddenPayloadEnum,
}
impl HiddenPayload {
  pub fn new(id: Uuid, hidden: HiddenPayloadEnum) -> Self { Self { id, hidden } }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum HiddenPayloadEnum {
  Hidden,
  UnHidden,
}
impl Default for HiddenPayloadEnum {
  fn default() -> Self { Self::Hidden }
}

/// A payload for editing an item
#[derive(Default, Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
#[schema(default = EditItemPayload::default, example=EditItemPayload::default)]
#[serde(rename_all = "camelCase")]
pub struct EditItemPayload {
  #[garde(skip)]
  pub id:       Uuid,
  #[garde(dive)]
  pub title:    Title,
  #[garde(skip)]
  pub text:     String, // todo(validate)
  #[garde(skip)]
  pub category: String, // todo(validate)
}

impl EditItemPayload {
  pub fn new(id: Uuid, title: &str, text: &str, category: &str) -> Self {
    Self { id, title: title.into(), text: text.into(), category: category.into() }
  }
}

/// A payload for getting items by different sorting methods
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum ItemKind {
  Ranked,
  Newest,
  RankedShow,
  Ask,
  BySiteDomain,
  ByUser,
  ByDay,
}
impl Default for ItemKind {
  fn default() -> Self { Self::Ranked }
}

// ranked, newest, rankedshow, newestshow, rankedask, sitedomain, submittedbyuser, rankedbyday,
// farovitedbypage, hiddenbypage, upvotedbypage,
