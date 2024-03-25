//! implement AuthnBackend - tell the library how to authenticate users
use axum::async_trait;
use axum_login::{AuthnBackend, UserId};
use db::DbPool;
use oauth2::{
  basic::{BasicClient, BasicRequestTokenError},
  url::Url,
  AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, TokenResponse,
  TokenUrl,
};

use super::{auth_user::User, credentials::Credentials};
use crate::{auth::UserInfo, error::ApiError, ApiResult};

#[derive(Debug, Clone)]
pub struct Backend {
  db:     DbPool,
  client: BasicClient,
}

impl Backend {
  pub fn new(db: DbPool, client: BasicClient) -> Self { Self { db, client } }

  pub fn new_with_default_client(db: DbPool) -> Self {
    let client = BasicClient::new(
      ClientId::new("client_id".to_string()),
      Some(ClientSecret::new("client_secret".to_string())),
      AuthUrl::new("http://authorize".to_string()).unwrap(),
      Some(TokenUrl::new("http://token".to_string()).unwrap()),
    );
    // Set the URL the user will be redirected to after the authorization process.
    // .set_redirect_uri(RedirectUrl::new("http://redirect".to_string()).unwrap());

    Self { db, client }
  }

  pub fn authorize_url(&self) -> (Url, CsrfToken) {
    self.client.authorize_url(CsrfToken::new_random).url()
  }
}

#[async_trait]
impl AuthnBackend for Backend {
  type Credentials = Credentials;
  type Error = ApiError;
  type User = User;

  async fn authenticate(&self, creds: Self::Credentials) -> ApiResult<Option<Self::User>> {
    match creds {
      Self::Credentials::Password(password_creds) =>
        password_creds.authenticate_password(&self.db).await,
      Self::Credentials::OAuth(oauth_creds) =>
        oauth_creds.authenticate(&self.db, &self.client).await,
    }
  }

  async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
    let user = db::queries::get_user(&self.db, user_id).await?.map(User);
    Ok(user)
  }
}
