use db::models::user_favorite::FavoriteStateEnum;

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
  pub content_id: Uuid,
  pub vote_state: VoteState,
}
impl VotePayload {
  pub fn new(content_id: Uuid, vote: VoteState) -> Self { Self { content_id, vote_state: vote } }
}

/// A payload for favoriting on an item or comment
#[derive(Default, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(default = FavoritePayload::default, example=FavoritePayload::default)]
#[serde(rename_all = "camelCase")]
pub struct FavoritePayload {
  pub id:       Uuid,
  pub favorite: FavoriteStateEnum,
}
impl FavoritePayload {
  pub fn new(id: Uuid, favorite: FavoriteStateEnum) -> Self { Self { id, favorite } }
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

#[derive(Default, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum HiddenPayloadEnum {
  #[default]
  Hidden,
  UnHidden,
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
  #[garde(dive)]
  pub text:     Text,
  #[garde(skip)]
  pub category: ItemCategory,
}

impl EditItemPayload {
  pub fn new(id: Uuid, title: &str, text: &str, category: ItemCategory) -> Self {
    Self { id, title: title.into(), text: text.into(), category }
  }
}

/// A payload for getting items by different sorting methods
#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize, Serialize, ToSchema)]
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
// farovitedbypage, hiddenbypage, upvotedbypage,
