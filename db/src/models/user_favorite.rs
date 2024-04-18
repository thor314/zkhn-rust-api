use super::*;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct UserFavorite {
  pub id:        Ulid,
  pub username:  Username,
  /// comment or item
  pub item_type: String,
  pub item_id:   Ulid,
  pub date:      Timestamp,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
#[schema(default = FavoriteStateEnum::default, example = FavoriteStateEnum::default)]
pub enum FavoriteStateEnum {
  #[default]
  Favorite,
  None,
}
