use uuid::Uuid;

use crate::{error::DbError, models::user_vote::UserVote, DbPool, DbResult};

pub async fn get_user_vote_by_content_id(
  pool: &DbPool,
  username: &str,
  content_id: Uuid,
) -> DbResult<Option<UserVote>> {
  sqlx::query_as!(
    UserVote,
    "SELECT username, vote_type, content_id, parent_item_id, vote_state as \"vote_state: _\", \
     created FROM user_votes WHERE content_id = $1 and username = $2",
    content_id,
    username
  )
  .fetch_optional(pool)
  .await
  .map_err(DbError::from)
}
