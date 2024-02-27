use anyhow::Context;
use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
// use db::models::comment::Comment;
use db::models::user::User;
use uuid::Uuid as Uid;

use crate::{error::MyError, DbPool};

#[derive(Debug, Clone)]
// Newtype since cannot derive traits for types defined in other crates
struct UserNewType(User);

//
impl AuthUser for UserNewType {
  type Id = Uid;

  fn id(&self) -> Self::Id { self.0.id }

  fn session_auth_hash(&self) -> &[u8] { self.0.password_hash.as_bytes() }
}

#[derive(Clone)]
struct Backend {
  db_pool: DbPool,
}

struct Credentials {
  id: Uid,
}

#[async_trait]
impl AuthnBackend for Backend {
  type Credentials = Credentials;
  type Error = MyError;
  type User = UserNewType;

  async fn authenticate(
    &self,
    Credentials { id }: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    let user = db::get_user_from_id(&self.db_pool, id).await.map(UserNewType);
    Ok(user)
  }

  async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
    let user = db::get_user_from_id(&self.db_pool, *user_id).await.map(UserNewType);
    Ok(user)
  }
}
