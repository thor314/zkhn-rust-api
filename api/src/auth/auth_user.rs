//! A simplified auth struct for testing auth
//! ref: https://github.com/maxcountryman/axum-login/blob/main/examples/multi-auth/src/users.rs
use axum::Router;
use axum_login::{tower_sessions::SessionManagerLayer, AuthUser};
use db::{AuthToken, DbPool, PasswordHash, Username};
use serde::{Deserialize, Serialize};
use tower_sessions_sqlx_store::PostgresStore;
use uuid::Uuid;

// /// A simplified User struct, to be used for authorization.
// #[derive(Clone, Serialize, Deserialize)]
// pub struct User {
//   pub username:      Username,
//   pub password_hash: Option<PasswordHash>,
//   pub auth_token:    Option<AuthToken>,
// }

// }

// impl From<db::models::user::User> for User {
//   fn from(user: db::models::user::User) -> Self {
//     Self {
//       username:      user.username,
//       password_hash: Some(user.password_hash),
//       auth_token:    user.auth_token,
//     }
//   }
// }
