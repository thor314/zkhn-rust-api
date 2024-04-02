use tracing::{debug, warn};
use uuid::Uuid;

use crate::{
  error::DbError,
  models::{comment::Comment, item::Item},
  DbPool, DbResult, Title, Username,
};

/// Create a new item in the database.
pub async fn create_item(pool: &DbPool, item: &Item) -> DbResult<()> {
  debug!("create_item with: {item:?}");
  let mut tx = pool.begin().await?;

  let Item { id, username, title, item_type, url, domain, text, item_category, .. } = item.clone();

  let result = sqlx::query!(
    "INSERT INTO items
    ( id,
    username,
    title,
    item_type,
    url,
    domain,
    text,
    item_category
  ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    id,
    username.0,
    title.0,
    item_type,
    url,
    domain,
    text,
    item_category,
  )
  .execute(&mut *tx)
  .await;

  // todo(karma): increment

  if let Err(e) = &result {
    // unwrap is safe; error is always db error kinded
    if e.as_database_error().expect("expected db error").is_unique_violation() {
      tx.rollback().await?;
      warn!("item already exists");
      return Err(DbError::Conflict);
    } else {
      tracing::error!("error creating item: {e}");
    }
  }
  let _ = result?;
  tx.commit().await?;
  Ok(())
}

// pub async fn get_item(pool: &DbPool, item_id: Uuid) -> DbResult<Option<Item>> {
//   sqlx::query_as!(
//     Item,
//     "SELECT
//       id,
//       username as \"username: Username\",
//       title as \"title: Title\",
//       item_type,
//       url,
//       domain,
//       text,
//       points,
//       score,
//       comment_count,
//       item_category,
//       created,
//       dead
//     FROM items WHERE id = $1",
//     item_id
//   )
//   .fetch_optional(pool)
//   .await
//   .map_err(DbError::from)
// }

// pub async fn delete_item(pool: &DbPool, item_id: Uuid) -> DbResult<()> {
//   sqlx::query!("DELETE FROM items WHERE id = $1", item_id)
//     .execute(pool)
//     .await
//     .map_err(DbError::from)?;
//   // todo delete comments, adjust karma

//   Ok(())
// }

// pub async fn update_item_category(
//   pool: &DbPool,
//   item_id: Uuid,
//   item_category: &str,
// ) -> DbResult<()> {
//   sqlx::query!(
//     "UPDATE items
//     SET item_category = $1
//     WHERE id = $2",
//     item_category,
//     item_id
//   )
//   .execute(pool)
//   .await
//   .map_err(DbError::from)?;

//   Ok(())
// }

// pub(crate) async fn decrement_item_comment_count_by(
//   pool: &sqlx::Pool<sqlx::Postgres>,
//   item_id: Uuid,
//   len: i32,
// ) -> DbResult<()> {
//   sqlx::query!(
//     "UPDATE items
//     SET comment_count = comment_count - $1
//     WHERE id = $2",
//     len,
//     item_id
//   )
//   .execute(pool)
//   .await?;
//   Ok(())
// }
