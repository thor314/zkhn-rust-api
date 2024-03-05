#![allow(dead_code)]
// docs: https://docs.rs/sqlx/latest/sqlx/attr.test.html
// examples: https://github.com/launchbadge/sqlx/blob/main/examples/postgres/axum-social-with-tests/tests/user.rs

// use rstest::{fixture, rstest};

use db::*;
use sqlx::PgPool;
use uuid::Uuid;

static INIT: std::sync::Once = std::sync::Once::new();
fn _setup_test_tracing() {
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
async fn integration_test(pool: PgPool) -> sqlx::Result<()> {
  let id = Uuid::new_v4();
  let user = get_user_by_id(&pool, id).await.unwrap();
  assert!(user.is_none());

  let username = "testuser";
  let user = get_user_by_username(&pool, username).await.unwrap();
  assert!(user.is_none());

  Ok(())
}
