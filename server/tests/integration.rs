use api::*;
use reqwest::Client;
use serial_test::serial;

use self::integration_utils::cargo_shuttle_run;
use crate::integration_utils::send;

pub const WEBSERVER_URL: &str = "http://localhost:8000";

mod integration_utils;

#[tokio::test]
#[serial]
async fn user_crud() {
  let mut _child_guard = cargo_shuttle_run().await;
  let c = Client::builder().cookie_store(true).build().unwrap();

  send(&c, "", "GET", "users/alice", 404, "00").await;
  send(&c, "", "GET", "users/authenticate/alice", 401, "11").await;
  send(&c, UserPayload::default(), "POST", "users", 200, "1").await;
  send(&c, "", "GET", "users/authenticate/alice", 401, "44").await;
  send(&c, UserPayload::default(), "POST", "users", 409, "2").await;
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "3").await;
  send(&c, "", "GET", "users/authenticate/alice", 200, "22").await;
  send(&c, "", "GET", "users/authenticate/bob", 403, "33").await;
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "4").await;
  send(&c, CredentialsPayload::default(), "POST", "users/logout", 200, "5").await;
  send(&c, "", "GET", "users/authenticate/alice", 401, "44").await;
  send(&c, CredentialsPayload::default(), "POST", "users/logout", 200, "6").await;
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "7").await;
  send(&c, UserUpdatePayload::default(), "PUT", "users", 200, "8").await;
  send(&c, UserUpdatePayload::default(), "PUT", "users", 200, "9").await;
  send(&c, UserUpdatePayload::default(), "PUT", "users", 200, "0").await;
  send(&c, "", "PUT_EMPTY", "users/reset-password-link/alice", 200, "a").await;
  send(&c, "", "PUT_EMPTY", "users/reset-password-link/alice", 200, "b").await;
  send(&c, ChangePasswordPayload::default(), "PUT", "users/change-password", 200, "c").await;
  send(&c, ChangePasswordPayload::default(), "PUT", "users/change-password", 200, "d").await;
  send(&c, "", "GET", "users/alice", 200, "e").await;
}

// #[tokio::test]
// #[serial]
// async fn items_crud() {
//   let mut _child_guard = cargo_shuttle_run().await;
//   let c = Client::builder().cookie_store(true).build().unwrap();
//   send(&c, UserPayload::default(), "POST", "users", 200, "1").await;
//   send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "2").await;
//   // send(&c, )
// }
