use axum::http::StatusCode;
use tracing::{debug, error};

use crate::{auth::users::AuthSession, ApiError, ApiResult, CredentialsPayload};

/// Internal login logic.
/// Isolate from the login handler to maintain consistency with axum-login style example.
pub async fn login_post_internal(
  mut auth_session: AuthSession,
  creds: CredentialsPayload,
) -> ApiResult<StatusCode> {
  // authenticate never returns None, so unwrap is safe
  let user = auth_session.authenticate(creds.clone()).await?.unwrap();
  // update the session with the user's login
  auth_session.login(&user).await?;
  debug!("login success for user: {}", creds.username);
  Ok(StatusCode::OK)
}

/// Internal logout logic.
/// Isolate from the login handler to maintain consistency with axum-login style example.
///
/// Will only error if db fails to flush the session.
pub async fn logout_post_internal(mut auth_session: AuthSession) -> ApiResult<StatusCode> {
  match auth_session.logout().await {
    Ok(_r) => Ok(StatusCode::OK),
    Err(_e) => Err(ApiError::OtherISE("logout error: {_e:?}".to_string())),
  }
}

// hack - remove eventually
// REDIRECT version of the login handler
// /// Internal login logic.
// /// Isolate from the login handler to maintain consistency with axum-login style example.
// pub async fn login_post_internal(
//   mut auth_session: AuthSession,
//   creds: CredentialsPayload,
// ) -> ApiResult<Redirect> {
//   // verify the user's login credentials
//   let user = match auth_session.authenticate(creds.clone()).await {
//     // login success
//     Ok(Some(user)) => user,
//     // login incorrect password - reroute to login page
//     Ok(None) => {
//       let login_base_url = "/users/login";
//       let login_url = creds
//         .next
//         .map(|next| format!("{}?next={}", login_base_url, next))
//         .unwrap_or(login_base_url.to_string());

//       debug!("login incorrect password for user: {}", creds.username);
//       return Err(ApiError::IncorrectPassword("incorrect password".to_string()));
//     },
//     // internal authentication error
//     Err(e) => return Err(ApiError::OtherISE(format!("authentication logic error: {e:?}"))),
//   };

//   // update the session with the user's login
//   if auth_session.login(&user).await.is_err() {
//     // internal authentication error
//     debug!("login ise for user: {}", creds.username);
//     return Err(ApiError::OtherISE("authentication login logic error".to_string()));
//   }

//   let redirect_uri = creds.next.unwrap_or("/".to_string());
//   debug!("login success for user: {}, redirecting to {}", creds.username, redirect_uri);
//   println!("success");
//   Ok(Redirect::to(&redirect_uri))
// }
