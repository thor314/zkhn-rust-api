use uuid::Uuid;

use crate::{
  error::DbError,
  models::user_vote::{UserVote, VoteState},
  utils::now,
  DbPool, DbResult, Username,
};

pub async fn get_user_vote_by_content_id(
  pool: &DbPool,
  username: &str,
  content_id: Uuid,
) -> DbResult<Option<UserVote>> {
  sqlx::query_as!(
    UserVote,
    "SELECT username as \"username: Username\", vote_type, content_id, parent_item_id, vote_state \
     as \"vote_state: _\", created FROM user_votes WHERE content_id = $1 and username = $2",
    content_id,
    username
  )
  .fetch_optional(pool)
  .await
  .map_err(DbError::from)
}

/// submit an upvote on a comment in the db. Assume the user has not already upvoted the comment
/// (verified in API)
pub async fn submit_comment_vote(
  pool: &mut sqlx::Pool<sqlx::Postgres>,
  comment_id: Uuid,
  username: &str,
  parent_item_id: Uuid,
  vote_state: VoteState,
) -> DbResult<()> {
  let mut tx = pool.begin().await?;
  sqlx::query!(
    "INSERT INTO user_votes (username, vote_type, content_id, parent_item_id, vote_state, created)
         VALUES ($1, $2, $3, $4, $5, $6)",
    username,
    "comment",
    comment_id,
    parent_item_id,
    VoteState::Upvote as _,
    now().0,
  )
  .execute(&mut *tx)
  .await?;

  // Update comment points (adjust query if points are stored differently)
  sqlx::query!("UPDATE comments SET points = points + 1 WHERE id = $1", comment_id)
    .execute(&mut *tx)
    .await?;

  // Update user karma (implement logic here, assuming a `users` table with `karma` field)
  sqlx::query!("UPDATE users SET karma = karma + 1 WHERE username = $1", username,)
    .execute(&mut *tx)
    .await?;

  tx.commit().await?;
  Ok(())
}
