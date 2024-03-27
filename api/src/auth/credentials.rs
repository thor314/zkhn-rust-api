use db::{DbPool, Password, Username};
use oauth2::{
  basic::BasicClient, http::header::USER_AGENT, reqwest::async_http_client, AuthorizationCode,
  CsrfToken, TokenResponse,
};
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

use self::{oauth_creds::OAuthCreds, password_creds::PasswordCreds};
use super::auth_user;
use crate::{
  auth::{auth_user::User, UserInfo},
  error::ApiError,
  ApiResult,
};

/// Users may log in either via password or via OAuth.
// ref: https://github.com/maxcountryman/axum-login/blob/main/examples/multi-auth/src/users.rs#L57
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Credentials {
  Password(PasswordCreds),
  OAuth(OAuthCreds),
}

pub mod password_creds {

  use db::password::verify_password;
  use tracing::debug;

  use super::*;
  /// Credentials for logging in with a username and password.
  // ref: https://github.com/maxcountryman/axum-login/blob/main/examples/multi-auth/src/users.rs#L63
  #[derive(Debug, Clone, Deserialize, Serialize)]
  pub struct PasswordCreds {
    pub username: Username,
    pub password: Password,
    /// Where to redirect the user after login
    pub next:     Option<String>,
  }

  impl PasswordCreds {
    pub fn new(username: &str, password: &str, next: Option<String>) -> Self {
      let username = Username(username.to_string());
      let password = Password(password.to_string());
      Self { username, password, next }
    }

    /// Returns:
    /// - Ok(Some(user)) - if the user was found and the password was correct
    /// - Ok(None) - if the user was not found, or if the user was found but the password was
    ///   incorrect
    /// - Err(_) - if there was an error with the database
    pub async fn authenticate_password(&self, pool: &DbPool) -> ApiResult<Option<User>> {
      debug!("authenticating user: {:?}", self);
      let user = db::queries::get_user(pool, &self.username)
        .await?
        .filter(|user| verify_password(&user.password_hash, &self.password).is_ok())
        .inspect(|user| tracing::debug!("authenticated user: {:?}", user))
        .map(User);

      Ok(user)
    }
  }
}

pub mod oauth_creds {

  use super::*;

  /// Credentials for logging in with an OAuth code.
  // ref: https://github.com/maxcountryman/axum-login/blob/main/examples/multi-auth/src/users.rs#L70
  #[derive(Debug, Clone, Deserialize, Serialize)]
  pub struct OAuthCreds {
    pub code:      String,
    pub old_state: CsrfToken,
    pub new_state: CsrfToken,
    // pub next:      Option<String>, // todo
  }

  impl OAuthCreds {
    /// Returns:
    /// - Ok(Some(user)) - if the user was found and the password was correct
    /// - Ok(None) - if the user was not found, or if the user was found but the password was
    ///   incorrect
    /// - Err(_) - if there was an error with the database
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
                    .map_err(ApiError::OAuthRequestFailure)?
                    .json::<UserInfo>() // todo: wtf
                    .await
                    .map_err(|e| ApiError::OAuthBadGateway(e.to_string()))?;

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
