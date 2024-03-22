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
use crate::{auth::UserInfo, error::ApiError};

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

  async fn authenticate(
    &self,
    creds: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    match creds {
      Self::Credentials::Password(password_cred) => {
        let user = db::queries::get_user(&self.db, &password_cred.username).await?;

        // Verifying the password is blocking and potentially slow, so we'll do so via
        // `spawn_blocking`.
        // task::spawn_blocking(|| {
        //   // We're using password-based authentication: this works by comparing our form
        //   // input with an argon2 password hash.
        //   let filtered_user = user.filter(|user| {
        //     let Some(ref password) = user.password else {
        //       return false;
        //     };

        //     let stored_password_hash = user.password_hash;
        //     verify_password(stored_password_hash, password_cred.password).is_ok()
        //   });

        //   Ok(filtered_user)
        // })
        // .await?
        todo!()
      },

      Self::Credentials::OAuth(oauth_creds) => {
        // Ensure the CSRF state has not been tampered with.
        if oauth_creds.old_state.secret() != oauth_creds.new_state.secret() {
          return Ok(None);
        };

        // Process authorization code, expecting a token response back.
        let token_res = self
          .client
          .exchange_code(AuthorizationCode::new(oauth_creds.code))
          .request_async(async_http_client)
          .await
          .map_err(Self::Error::OAuth2)?;

        // Use access token to request user info.
        let user_info = reqwest::Client::new() // todo: why not reuse backend's client
        // let user = self.client
                    .get("https://api.github.com/user")
                    .header(USER_AGENT.as_str(), "axum-login") // See: https://docs.github.com/en/rest/overview/resources-in-the-rest-api?apiVersion=2022-11-28#user-agent-required
                    .header(
                        AUTHORIZATION.as_str(),
                        format!("Bearer {}", token_res.access_token().secret()),
                    )
                    .send()
                    .await
                    .map_err(ApiError::AuthReqwest)?
                    .json::<UserInfo>() // todo: wtf
                    .await
                    .map_err(ApiError::AuthReqwest)?;

        // Persist user in our database so we can use `get_user`.
        // let user = db::queries::update_access_token(&self.db, username, access_token).await?;
        // let user = sqlx::query_as(
        //   r#"
        //             insert into users (username, access_token)
        //             values (?, ?)
        //             on conflict(username) do update
        //             set access_token = excluded.access_token
        //             returning *
        //             "#,
        // )
        // .bind(user_info.login)
        // .bind(token_res.access_token().secret())
        // .fetch_one(&self.db)
        // .await
        // .map_err(Self::Error::Sqlx)?;

        // Ok(Some(user))
        todo!()
      },
    }
  }

  async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
    Ok(db::queries::get_user(&self.db, user_id).await.unwrap().map(|u| u.into()))
  }
}
