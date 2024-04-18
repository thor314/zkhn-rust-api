use db::{models::user_favorite::FavoriteStateEnum, Ulid};

use super::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(default = CreateItemPayload::default, example=CreateItemPayload::default)]
pub struct CreateItemPayload {
  #[garde(dive)]
  pub title:           Title,
  #[garde(skip)]
  item_type:           ItemType,
  #[garde(dive)]
  text_or_url_content: TextOrUrl,
  #[garde(skip)]
  item_category:       ItemCategory,
}

impl CreateItemPayload {
  pub async fn into_item(self, username: Username) -> Item {
    Item::new(username, self.title, self.item_type, self.text_or_url_content, self.item_category)
  }

  /// convenience method for testing
  pub fn new(
    title: &str,
    item_type: ItemType,
    is_text: bool,
    text_or_url_content: TextOrUrl,
    item_category: ItemCategory,
  ) -> ApiResult<Self> {
    let title = title.into();

    let item_payload = Self { title, item_type, text_or_url_content, item_category };
    item_payload.validate(&())?;
    Ok(item_payload)
  }
}

/// A payload for voting on an item or comment
#[derive(Default, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(default = VotePayload::default, example=VotePayload::default)]
#[serde(rename_all = "camelCase")]
pub struct VotePayload {
  pub content_id: Ulid,
  pub vote_state: VoteState,
}
impl VotePayload {
  pub fn new(content_id: &Ulid, vote: VoteState) -> Self {
    Self { content_id: content_id.clone(), vote_state: vote }
  }
}

/// A payload for favoriting on an item or comment
#[derive(Default, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(default = FavoritePayload::default, example=FavoritePayload::default)]
#[serde(rename_all = "camelCase")]
pub struct FavoritePayload {
  pub id:       Ulid,
  pub favorite: FavoriteStateEnum,
}
impl FavoritePayload {
  pub fn new(id: &Ulid, favorite: FavoriteStateEnum) -> Self { Self { id: id.clone(), favorite } }
}

/// A payload for editing an item
#[derive(Default, Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
#[schema(default = EditItemPayload::default, example=EditItemPayload::default)]
#[serde(rename_all = "camelCase")]
pub struct EditItemPayload {
  #[garde(dive)]
  pub id:        Ulid,
  #[garde(dive)]
  pub title:     Title,
  #[garde(dive)]
  pub text:      Text,
  #[garde(skip)]
  pub category:  ItemCategory,
  #[garde(skip)]
  pub item_type: ItemType,
}

impl EditItemPayload {
  pub fn new(
    id: &Ulid,
    title: &str,
    text: &str,
    category: ItemCategory,
    item_type: ItemType,
  ) -> Self {
    Self { id: id.clone(), title: title.into(), text: text.into(), category, item_type }
  }
}

/// A payload for getting items by different sorting methods
#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
#[schema(default = ItemKind::default, example=ItemKind::default)]
#[serde(rename_all = "camelCase")]
pub enum ItemKind {
  #[default]
  Ranked,
  Newest,
  RankedShow,
  Ask,
  BySiteDomain,
  ByUser,
  ByDay,
}

// ranked, newest, rankedshow, newestshow, rankedask, sitedomain, submittedbyuser, rankedbyday,
// farovitedbypage, upvotedbypage,
