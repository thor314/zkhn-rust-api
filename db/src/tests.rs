#![allow(dead_code)]
// docs: https://docs.rs/sqlx/latest/sqlx/attr.test.html
// examples: https://github.com/launchbadge/sqlx/blob/main/examples/postgres/axum-social-with-tests/tests/user.rs

// use rstest::{fixture, rstest};

use sqlx::{PgConnection, PgPool, Row};
use uuid::Uuid;

use crate::{
  models::{comment::Comment, item::Item, user::User},
  queries::*,
  Email,
};

static INIT: std::sync::Once = std::sync::Once::new();
fn setup_test_tracing() {
  use tracing::Level;
  use tracing_subscriber::FmtSubscriber;

  INIT.call_once(|| {
    let subscriber =
      FmtSubscriber::builder().with_max_level(Level::INFO).with_test_writer().finish();
    tracing::subscriber::set_global_default(subscriber).expect(
      "setting default subscriber
failed",
    );
  });
}

#[sqlx::test]
async fn user_item_comment_round_trip(pool: PgPool) -> sqlx::Result<()> {
  let mut users = (1i32..).map(|i| {
    User::new(
      format!("testuser{}", i),
      "testpassword".to_string(),
      Some(Email("testemail".to_string())),
      None,
    )
  });
  let user = users.next().unwrap();
  create_user(&pool, &user).await.unwrap();
  let gotten_user = get_user(&pool, &user.username).await.unwrap().unwrap();
  assert_eq!(user.username, gotten_user.username);

  let about = "testabout".to_string();
  update_user_about(&pool, &user.username, &about).await.unwrap();
  let gotten_about = get_user(&pool, &user.username).await.unwrap().unwrap().about.unwrap();
  assert_eq!(gotten_about.0, about);

  let user_items = get_user_items(&pool, &user.username).await.unwrap();
  assert!(user_items.is_empty());

  let mut items = (1i32..).map(|i| {
    Item::new(
      user.username.clone(),
      format!("testtitle{}", i),
      "news".to_string(),
      true,
      "text content".to_string(),
      "other".to_string(),
    )
  });
  let item = items.next().unwrap();
  insert_item(&pool, &item).await.unwrap();
  let gotten_category = get_item(&pool, item.id).await.unwrap().unwrap().item_category;
  assert_eq!(gotten_category, "other".to_string());

  let category = "paper".to_string();
  update_item_category(&pool, item.id, &category).await.unwrap();
  let gotten_category = get_item(&pool, item.id).await.unwrap().unwrap().item_category;
  assert_eq!(gotten_category, category);

  // todo: try to insert a comment
  let mut comments = (1i32..).map(|i| {
    Comment::new(
      user.username.clone(),
      item.id,
      item.title.clone(),
      true,
      None,
      None,
      format!("testcomment{}", i),
      false,
    )
  });
  let comment = comments.next().unwrap();
  insert_comment(&pool, &comment).await.unwrap();
  let comments_number = get_item(&pool, item.id).await.unwrap().unwrap().comment_count;
  assert_eq!(1, comments_number);

  delete_comment(&pool, comment.id, item.id).await.unwrap();
  let comments_number = get_item(&pool, item.id).await.unwrap().unwrap().comment_count;
  assert_eq!(0, comments_number);

  delete_item(&pool, item.id).await.unwrap();
  let gotten_item = get_item(&pool, item.id).await.unwrap();
  assert!(gotten_item.is_none());

  delete_user(&pool, &user.username).await.unwrap();
  let gotten_user = get_user(&pool, &user.username).await.unwrap();
  assert!(gotten_user.is_none());

  // todo: test insert comment for user fails if user does not exist

  Ok(())
}
