use super::*;

pub async fn get_assert_item_vote(
  pool: &DbPool,
  username: &Username,
  id: Uuid,
) -> DbResult<UserVote> {
  get_item_vote(pool, username, id).await?.ok_or(DbError::NotFound("vote".into()))
}

pub async fn get_item_vote(
  pool: &DbPool,
  username: &Username,
  id: Uuid,
) -> DbResult<Option<UserVote>> {
  sqlx::query_as!(
    UserVote,
    "SELECT 
   username as \"username: Username\", 
    vote_type as \"vote_type: ItemOrComment\", 
    content_id, 
    parent_item_id, 
    vote_state as \"vote_state: VoteState\", 
    created 
    FROM user_votes WHERE content_id = $1 and username = $2",
    id,
    username.0
  )
  .fetch_optional(pool)
  .await
  .map_err(DbError::from)
}

/// Submit an vote on an item. Assume the api has already checked for repeatedly submitted
/// {up,down,un}votes.
///
/// Insert the vote into the database, updating the item_submitter's karma and the item's points.
pub async fn vote_on_item(
  pool: DbPool,
  item_id: Uuid,
  username: Username,
  vote_state: VoteState,
  preexisting_vote: Option<UserVote>,
) -> DbResult<()> {
  let mut tx = pool.begin().await?;

  // insert the vote in the votes table
  sqlx::query!(
    "INSERT INTO user_votes (
    username, 
    vote_type, 
    content_id, 
    vote_state, 
    created 
  ) VALUES ($1, $2, $3, $4, $5)",
    username.0,
    ItemOrComment::Item as ItemOrComment,
    item_id,
    vote_state.clone() as VoteState,
    now().0
  )
  .execute(&mut *tx)
  .await?;

  // Update item points and get the item submitter username
  let increment_value = {
    let pre_existing =
      preexisting_vote.as_ref().map(|v| i8::from(v.vote_state)).unwrap_or_default();
    i8::from(vote_state) - pre_existing
  } as i32;

  let submitter = sqlx::query!(
    "UPDATE items SET points = points + $1 WHERE id = $2 
    RETURNING username as \"username: Username\"",
    increment_value,
    item_id
  )
  .fetch_one(&mut *tx)
  .await?
  .username;

  sqlx::query!(
    "UPDATE users SET karma = karma + $1 WHERE username = $2",
    increment_value,
    submitter.0
  )
  .execute(&mut *tx)
  .await?;

  Ok(tx.commit().await?)
}

pub async fn get_user_votes_on_items_after(
  pool: &DbPool,
  username: &Username,
  after: Timestamp,
  page: Page, // todo
) -> DbResult<Vec<UserVote>> {
  sqlx::query_as!(
    UserVote,
    "SELECT 
    username as \"username: Username\", 
    vote_type as \"vote_type: ItemOrComment\", 
    content_id, 
    parent_item_id, 
    vote_state as \"vote_state: VoteState\", 
    created 
    FROM user_votes WHERE username = $1 AND created > $2 
    and vote_type = 'item' 
    ORDER BY created DESC",
    username.0,
    after.0
  )
  .fetch_all(pool)
  .await
  .map_err(DbError::from)
}

// pub async fn get_user_vote_by_content_id(
//   pool: &DbPool,
//   username: &str,
//   content_id: Uuid,
// ) -> DbResult<Option<UserVote>> {
//   sqlx::query_as!(
//     UserVote,
//     "SELECT username as \"username: Username\", vote_type, content_id, parent_item_id, vote_state
// \      as \"vote_state: _\", created FROM user_votes WHERE content_id = $1 and username = $2",
//     content_id,
//     username
//   )
//   .fetch_optional(pool)
//   .await
//   .map_err(DbError::from)
// }

// /// submit an upvote on a comment in the db. Assume the user has not already upvoted the comment
// /// (verified in API)
// pub async fn submit_comment_vote(
//   pool: &mut sqlx::Pool<sqlx::Postgres>,
//   comment_id: Uuid,
//   username: &str,
//   parent_item_id: Uuid,
//   vote_state: VoteState,
// ) -> DbResult<()> {
//   let mut tx = pool.begin().await?;
//   sqlx::query!(
//     "INSERT INTO user_votes (username, vote_type, content_id, parent_item_id, vote_state,
// created)          VALUES ($1, $2, $3, $4, $5, $6)",
//     username,
//     "comment",
//     comment_id,
//     parent_item_id,
//     VoteState::Upvote as _,
//     now().0,
//   )
//   .execute(&mut *tx)
//   .await?;

//   // Update comment points (adjust query if points are stored differently)
//   sqlx::query!("UPDATE comments SET points = points + 1 WHERE id = $1", comment_id)
//     .execute(&mut *tx)
//     .await?;

//   // Update user karma (implement logic here, assuming a `users` table with `karma` field)
//   sqlx::query!("UPDATE users SET karma = karma + 1 WHERE username = $1", username,)
//     .execute(&mut *tx)
//     .await?;

//   tx.commit().await?;
//   Ok(())
// }

// /// Create a new item in the database.
// pub async fn create_item(pool: &DbPool, item: &Item) -> DbResult<()> {
//   debug!("create_item with: {item:?}");
//   let mut tx = pool.begin().await?;

//   let Item { id, username, title, item_type, url, domain, text, item_category, .. } =
// item.clone();

//   sqlx::query!(
//     "INSERT INTO items
//     ( id,
//     username,
//     title,
//     item_type,
//     url,
//     domain,
//     text,
//     item_category
//   ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
//     id,
//     username.0,
//     title.0,
//     item_type as ItemType,
//     url.map(|s| s.0),
//     domain.map(|s| s.0),
//     text.map(|s| s.0),
//     item_category as ItemCategory,
//   )
//   .execute(&mut *tx)
//   .await?;

//   sqlx::query!(
//     "UPDATE users
//     SET karma = karma + 1
//     WHERE username = $1",
//     username.0
//   )
//   .execute(&mut *tx)
//   .await?;

//   Ok(tx.commit().await?)
// }
