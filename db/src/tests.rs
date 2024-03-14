#![allow(dead_code)]
// docs: https://docs.rs/sqlx/latest/sqlx/attr.test.html
// examples: https://github.com/launchbadge/sqlx/blob/main/examples/postgres/axum-social-with-tests/tests/user.rs

// use rstest::{fixture, rstest};

use sqlx::{PgConnection, PgPool, Row};
use uuid::Uuid;

use crate::{models::user::User, queries::*};

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
async fn user_creation(pool: PgPool) -> sqlx::Result<()> {
  let mut users = (1i32..).map(|i| {
    User::new(format!("testuser{}", i), "testpassword".to_string(), "testemail".to_string(), None)
  });
  let user = users.next().unwrap();
  insert_user(&pool, &user).await.unwrap();
  let gotten_user = get_user(&pool, &user.username).await.unwrap().unwrap();
  assert_eq!(user.username, gotten_user.username);

  let about = "testabout".to_string();
  update_user_about(&pool, &user.username, &about).await.unwrap();
  let gotten_about = get_user(&pool, &user.username).await.unwrap().unwrap().about.unwrap();
  assert_eq!(gotten_about, about);

  let user_comments = get_user_comments(&pool, &user.username).await.unwrap();
  assert!(user_comments.is_empty());
  // todo: try to insert a comment

  delete_user(&pool, &user.username).await.unwrap();
  let gotten_user = get_user(&pool, &user.username).await.unwrap();
  assert!(gotten_user.is_none());

  // todo: test insert comment for user fails if user does not exist

  Ok(())
}
