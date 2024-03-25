use db::{DbPool, Password, Username};
use oauth2::{
  basic::BasicClient, http::header::USER_AGENT, reqwest::async_http_client, AuthorizationCode,
  CsrfToken, TokenResponse,
};
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;

use self::{oauth_creds::OAuthCreds, password_creds::PasswordCreds};
use super::auth_user;
use crate::{
  auth::{User, UserInfo},
  error::ApiError,
  ApiResult,
};

/// Users may log in either via password or via OAuth.
#[derive(Debug, Clone, Deserialize)]
pub enum Credentials {
  Password(PasswordCreds),
  OAuth(OAuthCreds),
}

mod password_creds {

  use super::*;
  /// Credentials for logging in with a username and password.
  #[derive(Debug, Clone, Deserialize)]
  pub struct PasswordCreds {
    pub username: Username,
    pub password: Password,
    pub next:     Option<String>,
  }

  impl PasswordCreds {
    pub async fn authenticate(&self, pool: &DbPool) -> ApiResult<Option<User>> {
      let user = db::queries::get_user(pool, &self.username).await?;

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
    }
  }
}

mod oauth_creds {

  use super::*;

  /// Credentials for logging in with an OAuth code.
  #[derive(Debug, Clone, Deserialize)]
  pub struct OAuthCreds {
    pub code:      String,
    pub old_state: CsrfToken,
    pub new_state: CsrfToken,
  }

  impl OAuthCreds {
    pub async fn authenticate(
      &self,
      pool: &DbPool,
      client: &BasicClient,
    ) -> ApiResult<Option<User>> {
      // Ensure the CSRF state has not been tampered with.
      if self.old_state.secret() != self.new_state.secret() {
        return Ok(None);
      };

      // Process authorization code, expecting a token response back.
      let token_res = client
        .exchange_code(AuthorizationCode::new(self.code.clone()))
        .request_async(async_http_client)
        .await
        .map_err(ApiError::OAuth2)?;

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
    }
  }
}
