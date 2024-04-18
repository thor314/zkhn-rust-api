use super::*;
use crate::models::user_favorite::FavoriteStateEnum;

pub async fn get_assert_favorite(
  pool: &DbPool,
  username: &Username,
  content_id: &Ulid,
) -> DbResult<UserFavorite> {
  get_favorite(pool, username, content_id).await?.ok_or(DbError::NotFound("favorite".into()))
}

pub async fn get_favorite(
  pool: &DbPool,
  username: &Username,
  content_id: &Ulid,
) -> DbResult<Option<UserFavorite>> {
  sqlx::query_as!(
    UserFavorite,
    "SELECT id as \"id: Ulid\",  username, item_type, item_id as \"item_id: Ulid\", date
       FROM user_favorites WHERE item_id = $1 and username = $2",
    content_id.0,
    username.0
  )
  .fetch_optional(pool)
  .await
  .map_err(DbError::from)
}

/// get the favorite from the db
/// - if one exists and delete it
/// - else, create it
pub async fn favorite_item(
  pool: &DbPool,
  username: &Username,
  content_id: &Ulid,
) -> DbResult<FavoriteStateEnum> {
  match get_favorite(pool, username, content_id).await? {
    Some(favorite) => {
      sqlx::query!(
        "DELETE FROM user_favorites
      WHERE item_id = $1",
        favorite.item_id.0,
      )
      .execute(pool)
      .await?;
      Ok(FavoriteStateEnum::None)
    },
    None => {
      sqlx::query!(
        "INSERT INTO user_favorites (id, username, item_type, item_id, date)
         VALUES ($1, $2, $3, $4, $5)",
        Ulid::new().to_string(),
        username.0,
        "item",
        content_id.0,
        now().0,
      )
      .execute(pool)
      .await?;
      Ok(FavoriteStateEnum::Favorite)
    },
  }
}
