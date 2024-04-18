use super::*;

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
    id.to_string(),
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

  sqlx::query!("UPDATE users SET karma = karma + 1 WHERE username = $1", username.0)
    .execute(&mut *tx)
    .await?;

  Ok(tx.commit().await?)
}

pub async fn get_assert_item(pool: &DbPool, item_id: &Ulid) -> DbResult<Item> {
  debug!("get_assert_item with: {item_id:?}");
  get_item(pool, item_id).await?.ok_or(DbError::NotFound("item".into()))
}

pub async fn get_item(pool: &DbPool, item_id: &Ulid) -> DbResult<Option<Item>> {
  debug!("get_item with: {item_id:?}");
  sqlx::query_as!(
    Item,
    "SELECT
      id,
      username,
      title,
      item_type as \"item_type: ItemType\",
      url as \"url: Url\",
      domain as \"domain: Domain\",
      text as \"text: Text\",
      comment_count,
      points,
      score,
      item_category as \"item_category: ItemCategory\",
      created,
      dead
    FROM items WHERE id = $1",
    item_id.to_string()
  )
  .fetch_optional(pool)
  .await
  .map_err(DbError::from)
}

/// Return whether the item has any comments.
pub(crate) async fn item_has_comments(pool: &DbPool, id: &Ulid) -> bool {
  item_comment_count(pool, id).await > 0
}

pub(crate) async fn item_comment_count(pool: &DbPool, id: &Ulid) -> usize {
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
pub async fn delete_item(pool: &DbPool, item: &Item, username: &Username) -> DbResult<()> {
  let mut tx = pool.begin().await?;
  sqlx::query!("UPDATE users SET karma = karma - $1 WHERE username = $2", item.points, username.0)
    .execute(&mut *tx)
    .await?;

  sqlx::query!("DELETE FROM items WHERE id = $1", item.id.to_string()).execute(&mut *tx).await?;

  Ok(tx.commit().await?)
}

/// Get all items created after `start_date`
pub async fn get_items_created_after(
  pool: &DbPool,
  start_date: &Timestamp,
  page: &Page,
) -> DbResult<(Vec<Item>, usize)> {
  // .skip((page - 1) * config.itemsPerPage)
  sqlx::query_as!(
    Item,
    "SELECT
      id,
      username,
      title,
      item_type as \"item_type: ItemType\",
      url as \"url: Url\",
      domain as \"domain: Domain\",
      text as \"text: Text\",
      comment_count,
      points,
      score,
      item_category as \"item_category: ItemCategory\",
      created,
      dead
      FROM items WHERE created > $1 
      ORDER BY score DESC",
    start_date.0 // WHERE created > $1 AND id <> ALL($2)
  )
  .fetch_all(pool)
  .await
  .map(|items| {
    let items_len = items.len();
    // let items = todo(pagination)
    (items, items_len)
  })
  .map_err(DbError::from)
}

pub async fn edit_item(
  pool: &DbPool,
  item_id: &Ulid,
  title: &Title,
  category: ItemCategory,
  text: &Text,
) -> DbResult<()> {
  sqlx::query!(
    "UPDATE items
    SET title = $1, item_category = $2, text = $3
    WHERE id = $4",
    title.0,
    category as ItemCategory,
    text.0,
    item_id.to_string()
  )
  .execute(pool)
  .await?;

  Ok(())
}

// pub async fn update_item_category(
//   pool: &DbPool,
//   item_id: UlidWrapper,
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
