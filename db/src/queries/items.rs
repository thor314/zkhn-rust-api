use uuid::Uuid;

use crate::{
  error::DbError,
  models::{comment::Comment, item::Item},
  DbPool, DbResult, Title, Username,
};

pub async fn get_item(pool: &DbPool, item_id: Uuid) -> DbResult<Option<Item>> {
  sqlx::query_as!(
    Item,
    "SELECT 
      id,
      username as \"username: Username\",    
      title as \"title: Title\",  
      item_type,   
      url,         
      domain,      
      text,        
      points,      
      score,       
      comment_count,
      item_category,
      created,     
      dead
    FROM items WHERE id = $1",
    item_id
  )
  .fetch_optional(pool)
  .await
  .map_err(DbError::from)
}

pub async fn insert_item(pool: &DbPool, new_item: &Item) -> DbResult<()> {
  let mut tx = pool.begin().await?;

  let Item {
    id,
    username,
    title,
    item_type,
    url,
    domain,
    text,
    points,
    score,
    comment_count,
    item_category,
    created,
    dead,
  } = new_item.clone();

  sqlx::query!(
    "INSERT INTO items
    ( id,
    username,
    title,
    item_type,
    url,
    domain,
    text,
    points,
    score,
    comment_count,
    item_category,
    created,
    dead ) 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
    id,
    username.0,
    title.0,
    item_type,
    url,
    domain,
    text,
    points,
    score,
    comment_count,
    item_category,
    created.0,
    dead
  )
  .execute(&mut *tx)
  .await?;

  tx.commit().await?;
  // todo
  Ok(())
}

pub async fn delete_item(pool: &DbPool, item_id: Uuid) -> DbResult<()> {
  sqlx::query!("DELETE FROM items WHERE id = $1", item_id)
    .execute(pool)
    .await
    .map_err(DbError::from)?;
  // todo delete comments, adjust karma

  Ok(())
}

pub async fn update_item_category(
  pool: &DbPool,
  item_id: Uuid,
  item_category: &str,
) -> DbResult<()> {
  sqlx::query!(
    "UPDATE items
    SET item_category = $1
    WHERE id = $2",
    item_category,
    item_id
  )
  .execute(pool)
  .await
  .map_err(DbError::from)?;

  Ok(())
}

pub(crate) async fn decrement_item_comment_count_by(
  pool: &sqlx::Pool<sqlx::Postgres>,
  item_id: Uuid,
  len: i32,
) -> DbResult<()> {
  sqlx::query!(
    "UPDATE items
    SET comment_count = comment_count - $1
    WHERE id = $2",
    len,
    item_id
  )
  .execute(pool)
  .await?;
  Ok(())
}
