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
async fn basic_test(pool: PgPool) -> sqlx::Result<()> {
  let username = "testuser";
  let user = get_user(&pool, username).await.unwrap();
  assert!(user.is_none());

  Ok(())
}
