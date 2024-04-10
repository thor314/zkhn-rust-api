use super::*;

pub async fn get_assert_hidden(
  pool: &DbPool,
  username: &Username,
  id: Uuid,
) -> DbResult<UserHidden> {
  get_hidden(pool, username, id).await?.ok_or(DbError::NotFound("hidden".into()))
}

// todo(hidden) + get_assert_hidden
pub async fn get_hidden(
  pool: &DbPool,
  username: &Username,
  id: Uuid,
) -> DbResult<Option<UserHidden>> {
  todo!()
}

pub async fn get_hidden_item_ids_after(
  pool: &DbPool,
  username: &Username,
  start_date: Timestamp,
) -> DbResult<Vec<Uuid>> {
  // sqlx::query_as!(
  //   UserHidden,
  //   "SELECT
  //     item_id as \"item_id: Uuid\",
  //     FROM user_hidden WHERE username = $1",
  //   username.0
  // )
  // .fetch_all(pool)
  // .await
  // .map_err(DbError::from)
  todo!()
}
