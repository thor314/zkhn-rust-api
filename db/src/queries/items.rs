use uuid::Uuid;

use crate::{
  error::DbError,
  models::{comment::Comment, item::Item},
  DbPool, DbResult,
};

pub async fn get_item_by_id(pool: &DbPool, item_id: Uuid) -> DbResult<Option<Item>> {
  sqlx::query_as!(Item, "SELECT * FROM items WHERE id = $1", item_id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}
