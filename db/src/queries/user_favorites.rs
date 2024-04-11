use super::*;
use crate::models::user_favorite::FavoriteStateEnum;

pub async fn get_assert_favorite(
  pool: &DbPool,
  username: &Username,
  id: Uuid,
) -> DbResult<UserFavorite> {
  get_favorite(pool, username, id).await?.ok_or(DbError::NotFound("favorite".into()))
}

// todo(favorite) + get_assert_favorite
pub async fn get_favorite(
  pool: &DbPool,
  username: &Username,
  content_id: Uuid,
) -> DbResult<Option<UserFavorite>> {
  // todo!()
  Ok(None)
}

/// get the favorite from the db
/// - if one exists and delete it
/// - else, create it
pub async fn favorite_item(
  pool: &DbPool,
  username: &Username,
  content_id: Uuid,
) -> DbResult<FavoriteStateEnum> {
  match get_favorite(pool, username, content_id).await? {
    Some(favorite) => {
      // sqlx::query!(
      //   "DELETE FROM user_favorites
      // WHERE item_id = $1",
      //   favorite.item_id,
      // )
      // .execute(pool)
      // .await?;
      Ok(FavoriteStateEnum::None)
    },
    None => {
      // sqlx::query!(
      //   "INSERT INTO user_favorites (username, item_type, item_id, date)
      //    VALUES ($1, $2, $3, $4)",
      //   username,
      //   "item",
      //   content_id,
      //   now().0,
      // )
      // .execute(pool)
      // .await?;
      Ok(FavoriteStateEnum::Favorite)
    },
  }
}

// pub async fn get_user_favorite_by_username_and_item_id(
//   pool: &DbPool,
//   username: &str,
//   item_id: Uuid,
// ) -> DbResult<Option<UserFavorite>> {
//   sqlx::query_as!(
//     UserFavorite,
//     "SELECT username as \"username: Username\", item_type, item_id, date
//        FROM user_favorites WHERE item_id = $1 and username = $2",
//     item_id,
//     username
//   )
//   .fetch_optional(pool)
//   .await
//   .map_err(DbError::from)
// }

// /// Insert a new user favorite for comment
// pub async fn insert_or_delete_user_favorite_for_comment(
//   pool: &sqlx::Pool<sqlx::Postgres>,
//   username: &str,
//   maybe_favorite: Option<UserFavorite>,
//   comment_id: Uuid,
// ) -> DbResult<()> {
//   match maybe_favorite {
//     Some(favorite) => {
//       sqlx::query!(
//         "DELETE FROM user_favorites
//       WHERE item_id = $1",
//         favorite.item_id,
//       )
//       .execute(pool)
//       .await?;
//       Ok(())
//     },
//     None => {
//       sqlx::query!(
//         "INSERT INTO user_favorites (username, item_type, item_id, date)
//          VALUES ($1, $2, $3, $4)",
//         username,
//         "comment",
//         comment_id,
//         now().0,
//       )
//       .execute(pool)
//       .await?;
//       Ok(())
//     },
//   }
// }
