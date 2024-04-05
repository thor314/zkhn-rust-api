use uuid::Uuid;

use crate::{
  error::DbError, models::user_hidden::UserHidden, utils::now, DbPool, DbResult, Username,
};

pub async fn get_assert_hidden(pool: &DbPool, username: &Username, id: Uuid) -> DbResult<()> {
  get_hidden(pool, username, id).await?.ok_or(DbError::NotFound("hidden".into()))
}

// todo(hidden) + get_assert_hidden
pub async fn get_hidden(pool: &DbPool, username: &Username, id: Uuid) -> DbResult<Option<()>> {
  todo!()
}
