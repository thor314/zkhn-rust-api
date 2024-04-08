use super::*;

/// Geta user from the db.
pub async fn get_user(pool: &DbPool, username: &Username) -> DbResult<Option<User>> {
  trace!("get_user called w username: {username}");
  sqlx::query_as!(
    User,
    "SELECT username, 
            password_hash, 
            reset_password_token as \"reset_password_token: ResetPasswordToken\", 
            reset_password_token_expiration as \"reset_password_token_expiration: Timestamp\",  
            email as \"email: Email\", 
            created, 
            karma, 
            about as \"about: About\", 
            show_dead, 
            is_moderator, 
            banned 
     FROM users WHERE username = $1",
    username.0
  )
  .fetch_optional(pool)
  .await
  .map_err(DbError::from)
}

/// Get a user from the database by their username. Return an error on not found.
pub async fn get_assert_user(pool: &DbPool, username: &Username) -> DbResult<User> {
  trace!("get_assert_user called w username: {username}");
  get_user(pool, username).await?.ok_or(DbError::NotFound("user".into()))
}

/// Create a new user in the database.
pub async fn create_user(pool: &DbPool, new_user: &User) -> DbResult<()> {
  trace!("create_user with: {new_user:?}");
  let mut tx = pool.begin().await?;

  let User {
    username,
    password_hash,
    reset_password_token,
    reset_password_token_expiration,
    email,
    karma,
    ..
  } = new_user.clone();

  sqlx::query!(
    "INSERT INTO users
    ( username, password_hash, reset_password_token, reset_password_token_expiration, email ) 
    VALUES ($1, $2, $3, $4, $5)",
    username.0,
    password_hash.0,
    reset_password_token.map(|s| s.0),
    reset_password_token_expiration.map(|t| t.0),
    email.map(|s| s.0),
  )
  .execute(&mut *tx)
  .await?;

  tx.commit().await?;
  Ok(())
}

pub async fn update_user(
  pool: &DbPool,
  username: &Username,
  about: &Option<About>,
  email: &Option<Email>,
  show_dead: &Option<bool>,
) -> DbResult<()> {
  let mut tx = pool.begin().await?;
  if let Some(about) = about {
    sqlx::query!("UPDATE users SET about = $1 WHERE username = $2", about.0, username.0)
      .execute(&mut *tx)
      .await?;
  }
  if let Some(email) = email {
    sqlx::query!("UPDATE users SET email = $1 WHERE username = $2", email.0, username.0)
      .execute(&mut *tx)
      .await?;
  }
  if let Some(show_dead) = show_dead {
    sqlx::query!("UPDATE users SET show_dead = $1 WHERE username = $2", show_dead, username.0)
      .execute(&mut *tx)
      .await?;
  }

  trace!("update_user with: {username}");
  Ok(tx.commit().await?)
}

pub async fn update_user_password_token(
  pool: &DbPool,
  username: &Username,
  reset_password_token: &ResetPasswordToken,
  reset_password_token_expiration: &Timestamp,
) -> DbResult<()> {
  trace!("update_user_password_token with: {username}");
  sqlx::query!(
    "UPDATE users SET 
    reset_password_token = $1,  reset_password_token_expiration = $2  WHERE username = $3",
    reset_password_token.0,
    reset_password_token_expiration.0,
    username.0
  )
  .execute(pool)
  .await?;

  Ok(())
}

pub async fn update_user_password(
  pool: &DbPool,
  username: &Username,
  new_password_hash: &PasswordHash,
) -> DbResult<()> {
  trace!("update_user_password with: {username}");
  sqlx::query!(
    "UPDATE users SET  password_hash = $1 WHERE username = $2",
    new_password_hash.0,
    username.0,
  )
  .execute(pool)
  .await?;

  Ok(())
}

// pub async fn get_user_comments(pool: &DbPool, username: &Username) -> DbResult<Vec<Comment>> {
// trace!("get_user_comments with: {username}");
//   sqlx::query_as!(
//     Comment,
//     "SELECT
//     id,
//     username as \"username: Username\",
//     parent_item_id,
//     parent_item_title as \"parent_item_title: Title\",
//     comment_text as \"comment_text: CommentText\",
//     is_parent,
//     root_comment_id,
//     parent_comment_id,
//     children_count,
//     points,
//     created,
//     dead
//    FROM comments WHERE username = $1",
//     username.0
//   )
//   .fetch_all(pool)
//   .await?
// }

// pub async fn get_user_items(pool: &DbPool, username: &Username) -> DbResult<Vec<Item>> {
//   trace!("get_user_items with: {username}");
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
//     FROM items WHERE username = $1",
//     username.0
//   )
//   .fetch_all(pool)
//   .await?
// }
