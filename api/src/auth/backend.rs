// use async_trait::async_trait;
use axum::{
  async_trait,
  http::header::{AUTHORIZATION, USER_AGENT},
};
use axum_login::{AuthUser, AuthnBackend, UserId};
use db::{password::verify_password, DbPool};
use oauth2::{
  basic::{BasicClient, BasicRequestTokenError},
  reqwest::{async_http_client, AsyncHttpClientError},
  url::Url,
  AuthorizationCode, CsrfToken, TokenResponse,
};
// use password_auth::verify_password;
use serde::{Deserialize, Serialize};
// use sqlx::{FromRow, SqlitePool};
use tokio::task;
use tracing_subscriber::filter;

use super::credentials::Credentials;
use crate::{auth::UserInfo, error::ApiError, ApiResult};

#[derive(Debug, Clone)]
pub struct Backend {
  db:     DbPool,
  client: BasicClient,
}

impl Backend {
  pub fn new(db: DbPool, client: BasicClient) -> Self { Self { db, client } }

  pub fn authorize_url(&self) -> (Url, CsrfToken) {
    self.client.authorize_url(CsrfToken::new_random).url()
  }
}

#[async_trait]
impl AuthnBackend for Backend {
  type Credentials = Credentials;
  type Error = ApiError;
  type User = super::auth_user::User;

  async fn authenticate(&self, creds: Self::Credentials) -> ApiResult<Option<Self::User>> {
    match creds {
      Self::Credentials::Password(password_creds) => password_creds.authenticate(&self.db).await,
      Self::Credentials::OAuth(oauth_creds) =>
        oauth_creds.authenticate(&self.db, &self.client).await,
    }
  }

  async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
    Ok(db::queries::get_user(&self.db, user_id).await.unwrap().map(|u| u.into()))
  }
}
