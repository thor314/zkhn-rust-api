#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use api::*;
use reqwest::Client;
use serial_test::serial;

use self::integration_utils::cargo_shuttle_run;
use crate::integration_utils::send;

pub const WEBSERVER_URL: &str = "http://localhost:8000";

mod integration_utils;

// #[tokio::test]
// #[serial]
async fn user_crud() {
  let mut _child_guard = cargo_shuttle_run().await;
  let c = Client::builder().cookie_store(true).build().unwrap();

  send(&c, "", "GET", "users/alice", 404, "00").await;
  send(&c, "", "GET", "users/authenticate", 401, "11").await;
  send(&c, CreateUserPayload::default(), "POST", "users", 200, "1").await;
  send(&c, "", "GET", "users/authenticate", 401, "44").await;
  send(&c, CreateUserPayload::default(), "POST", "users", 409, "2").await;
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "3").await;
  send(&c, "", "GET", "users/authenticate", 200, "22").await;
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "4").await;
  send(&c, CredentialsPayload::default(), "POST", "users/logout", 200, "5").await;
  send(&c, "", "GET", "users/authenticate", 401, "44").await;
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

async fn user_bug() {
  let mut _child_guard = cargo_shuttle_run().await;
  let c = Client::builder().cookie_store(true).build().unwrap();
  let bob = CreateUserPayload::new("bob", "password", None, None).unwrap();
  send(&c, CreateUserPayload::default(), "POST", "users", 200, "1").await;
  send(&c, bob, "POST", "users", 200, "2").await;
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "3").await;
}


// #[tokio::test]
// #[serial]
async fn items_crud() {
  let mut _child_guard = cargo_shuttle_run().await;
  let c = Client::builder().cookie_store(true).build().unwrap();
  send(&c, CreateUserPayload::default(), "POST", "users", 200, "1").await;
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "2").await;
  let id: uuid::Uuid =
    send(&c, CreateItemPayload::default(), "POST", "items", 200, "3").await.json().await.unwrap();
  let fake_id = uuid::Uuid::new_v4();
  send(&c, "", "GET", &format!("items/{fake_id}?page=1"), 404, "40").await;
  send(&c, "", "GET", &format!("items/{id}?page=0"), 422, "41").await;
  send(&c, "", "GET", &format!("items/{id}?page=1"), 200, "4").await;
  let r: GetItemResponse =
    send(&c, "", "GET", &format!("items/{id}?page=2"), 200, "5").await.json().await.unwrap();
  assert!(r.comments.is_empty());

  // todo(test) vote_item
  // send(&c, "", "GET", &format!("items/{id}?2"), 200, "6").await;
  // send(&c, "", "GET", &format!("items/{id}?2"), 200, "7").await;
  // let upvote = VotePayload::new(id, VotePayloadEnum::Upvote);
  // let downvote = VotePayload::new(id, VotePayloadEnum::Downvote);
  // send(&c, upvote, "POST", "items/vote", 200, "8").await;

  // todo(test) favorite_item
  // todo(test) hide_item
}
