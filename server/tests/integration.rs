#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use api::*;
use db::models::user_vote::VoteState;
use reqwest::Client;
use serial_test::serial;
use uuid::Uuid;

use self::integration_utils::cargo_shuttle_run;
use crate::integration_utils::{send, send_get};

pub const WEBSERVER_URL: &str = "http://localhost:8000";

mod integration_utils;

#[tokio::test]
#[serial]
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
  send(&c, ChangePasswordPayload::default(), "PUT", "users/change-password", 401, "d").await;
  let new_payload =
    ChangePasswordPayload::new("alice", Some("new_password"), None, "password").unwrap();
  send(&c, new_payload, "PUT", "users/change-password", 200, "d").await;
  send(&c, "", "GET", "users/alice", 200, "e").await;
}

#[tokio::test]
#[serial]
async fn item_crud() {
  let mut _child_guard = cargo_shuttle_run().await;
  let c = Client::builder().cookie_store(true).build().unwrap();
  send(&c, CreateUserPayload::default(), "POST", "users", 200, "00").await;
  send(&c, CreateUserPayload::bob(), "POST", "users", 200, "01").await;

  // post item for alice as unauth: 403
  send(&c, CreateItemPayload::default(), "POST", "items", 403, "10").await;
  // post item for alice as alice: 200
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "11").await;
  let id = send_get::<Uuid>(&c, CreateItemPayload::default(), "POST", "items", 200, "12").await;
  // post item for alice as alice with invalid payload: 422
  send(&c, GetUserResponse::default(), "POST", "items", 422, "13").await;
  // let id: uuid::Uuid =
  // post duplicate item for alice as alice with invalid payload: 422
  send(&c, CreateItemPayload::default(), "POST", "items", 200, "14").await;
  // todo(testing) banned user post item: 401

  // get item
  // get item with fake id: 404
  let fake_id = uuid::Uuid::new_v4();
  send(&c, "", "GET", &format!("items/{fake_id}?page=1"), 404, "20").await;
  // get item with invalid id: 422
  send(&c, "", "GET", "items/&invalid_id&?page=1", 422, "21").await;
  // get item with with invalid page: 422
  send(&c, "", "GET", &format!("items/{id}?page=0"), 422, "22").await;
  // get item with with empty page: todo
  send(&c, "", "GET", &format!("items/{id}?page=3"), 422, "22a").await;
  // get real item: 200
  send(&c, "", "GET", &format!("items/{id}?page=1"), 200, "23").await;
  let r: GetItemResponse =
    send(&c, "", "GET", &format!("items/{id}?page=1"), 200, "24").await.json().await.unwrap();
  assert!(r.comments.is_empty());

  // vote item
  let upvote = VotePayload::new(id, VoteState::Upvote);
  let downvote = VotePayload::new(id, VoteState::Downvote);
  let unvote = VotePayload::new(id, VoteState::None);
  send(&c, upvote.clone(), "POST", "items/vote", 200, "6").await;
  send(&c, upvote.clone(), "POST", "items/vote", 409, "7").await;
  send(&c, downvote.clone(), "POST", "items/vote", 200, "8").await;
  send(&c, downvote.clone(), "POST", "items/vote", 409, "9").await;
  send(&c, unvote.clone(), "POST", "items/vote", 200, "01").await;
  send(&c, unvote.clone(), "POST", "items/vote", 409, "02").await;
  send(&c, downvote.clone(), "POST", "items/vote", 200, "03").await;
  send(&c, upvote.clone(), "POST", "items/vote", 200, "04").await;
  // next: get user karma
  // send(&c, "", "GET", &format!("items/{id}?2"), 200, "7").await;
  // let upvote = VotePayload::new(id, VotePayloadEnum::Upvote);
  // let downvote = VotePayload::new(id, VotePayloadEnum::Downvote);
  // send(&c, upvote, "POST", "items/vote", 200, "8").await;

  // todo(test) favorite_item
  // todo(test) hide_item

  // get edit item
  // edit item
  // get delete item
  // delete item
}
