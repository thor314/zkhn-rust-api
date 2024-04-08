use super::*;

pub async fn get_assert_favorite(pool: &DbPool, username: &Username, id: Uuid) -> DbResult<()> {
  get_favorite(pool, username, id).await?.ok_or(DbError::NotFound("favorite".into()))
}

// todo(favorite) + get_assert_favorite
pub async fn get_favorite(pool: &DbPool, username: &Username, id: Uuid) -> DbResult<Option<()>> {
  todo!()
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
