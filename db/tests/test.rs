#![allow(dead_code)]
// docs: https://docs.rs/sqlx/latest/sqlx/attr.test.html
// examples: https://github.com/launchbadge/sqlx/blob/main/examples/postgres/axum-social-with-tests/tests/user.rs

// use rstest::{fixture, rstest};
// use uuid::Uuid;

// use db::{models::user::User, queries::*};
// use sqlx::PgPool;

// static INIT: std::sync::Once = std::sync::Once::new();
// fn _setup_test_tracing() {
//   use tracing::Level;
//   use tracing_subscriber::FmtSubscriber;

//   INIT.call_once(|| {
//     let subscriber =
//       FmtSubscriber::builder().with_max_level(Level::INFO).with_test_writer().finish();
//     tracing::subscriber::set_global_default(subscriber).expect(
//       "setting default subscriber
// failed",
//     );
//   });
// }

