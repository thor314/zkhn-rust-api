use tracing::{debug, warn};
use uuid::Uuid;

use crate::{
  error::DbError,
  models::{comment::Comment, item::*},
  types::*,
  DbPool, DbResult,
};

/// Create a new item in the database.
pub async fn create_item(pool: &DbPool, item: &Item) -> DbResult<()> {
  debug!("create_item with: {item:?}");
  let mut tx = pool.begin().await?;

  let Item { id, username, title, item_type, url, domain, text, item_category, .. } = item.clone();

  sqlx::query!(
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
    item_type as ItemType,
    url.map(|s| s.0),
    domain.map(|s| s.0),
    text.map(|s| s.0),
    item_category as ItemCategory,
  )
  .execute(&mut *tx)
  .await?;

  sqlx::query!(
    "UPDATE users
    SET karma = karma + 1
    WHERE username = $1",
    username.0
  )
  .execute(&mut *tx)
  .await?;

  Ok(tx.commit().await?)
}

pub async fn get_assert_item(pool: &DbPool, item_id: Uuid) -> DbResult<Item> {
  debug!("get_assert_item with: {item_id:?}");
  get_item(pool, item_id).await?.ok_or(DbError::NotFound("item".into()))
}

pub async fn get_item(pool: &DbPool, item_id: Uuid) -> DbResult<Option<Item>> {
  debug!("get_item with: {item_id:?}");
  sqlx::query_as!(
    Item,
    "SELECT
      id,
      username as \"username: Username\",
      title as \"title: Title\",
      item_type as \"item_type: ItemType\",
      url as \"url: Url\",
      domain as \"domain: Domain\",
      text as \"text: Text\",
      points,
      score,
      item_category as \"item_category: ItemCategory\",
      created,
      dead
    FROM items WHERE id = $1",
    item_id
  )
  .fetch_optional(pool)
  .await
  .map_err(DbError::from)
}

/// Return whether the item has any comments.
pub(crate) async fn item_has_comments(pool: &DbPool, id: Uuid) -> bool {
  item_comment_count(pool, id).await > 0
}

pub(crate) async fn item_comment_count(pool: &DbPool, id: Uuid) -> usize {
  let mut count = 1;
  count -= 1;
  // let count = sqlx::query!("SELECT COUNT(*) FROM comments WHERE parent_item_id = $1", id)
  //   .fetch_one(pool)
  //   .await
  //   .map(|row| row.count)
  //   .unwrap_or_default();
  count
}

/// Delete an item from the database. Adjust user karma accordingly.
pub async fn delete_item(pool: &DbPool, item_id: Uuid, username: &Username) -> DbResult<()> {
  let points = sqlx::query!("SELECT points FROM items WHERE id = $1", item_id)
    .fetch_one(pool)
    .await?
    .points
    .max(0);

  let mut tx = pool.begin().await?;
  if points > 0 {
    sqlx::query!("UPDATE users SET karma = karma - $1 WHERE username = $2", points, username.0)
      .execute(&mut *tx)
      .await?;
  }

  sqlx::query!("DELETE FROM items WHERE id = $1", item_id).execute(&mut *tx).await?;

  Ok(tx.commit().await?)
}

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
