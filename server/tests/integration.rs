#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use api::*;
use db::models::{
  item::{Item, ItemCategory, ItemType},
  user_favorite::{FavoriteStateEnum, UserFavorite},
  user_vote::VoteState,
};
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

  // create an item by a dummy user
  let bob_creds = CredentialsPayload::new("bob", "password", None);
  send(&c, bob_creds.clone(), "POST", "users/login", 200, "01li").await;
  let bob_item_id =
    send_get::<Uuid>(&c, CreateItemPayload::default(), "POST", "items", 200, "01a").await;
  send(&c, bob_creds.clone(), "POST", "users/logout", 200, "01lo").await;

  // post item for alice as unauth: 401
  send(&c, CreateItemPayload::default(), "POST", "items", 401, "10").await;
  // post item for alice as alice: 200
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "11").await;
  let id = send_get::<Uuid>(&c, CreateItemPayload::default(), "POST", "items", 200, "12").await;
  // post item for alice as alice with invalid payload: 422
  send(&c, GetUserResponse::default(), "POST", "items", 422, "13").await;
  // post duplicate item for alice as alice with invalid payload: 422
  send(&c, CreateItemPayload::default(), "POST", "items", 200, "14").await;
  // todo(testing, banned) banned user post item: 401

  // get item with fake id: 404
  let fake_id = uuid::Uuid::new_v4();
  send(&c, "", "GET", &format!("items/{fake_id}?page=1"), 404, "20").await;
  // get item with invalid id: 400
  send(&c, "", "GET", "items/&invalid_id&?page=1", 400, "21").await;
  // get item with with invalid page: 422
  send(&c, "", "GET", &format!("items/{id}?page=0"), 422, "22").await;
  // get item with with negative page:
  send(&c, "", "GET", &format!("items/{id}?page=-1"), 422, "22a").await;
  // get item with without a page:
  send(&c, "", "GET", &format!("items/{id}"), 400, "22b").await;
  // get item with with too-high page: 200 (this is fine)
  send(&c, "", "GET", &format!("items/{id}?page=3"), 200, "22c").await;
  // get real item as alice: 200
  let r: GetItemResponse = send_get(&c, "", "GET", &format!("items/{id}?page=1"), 200, "24").await;
  // get real item as logged out: 200
  send(&c, CredentialsPayload::default(), "POST", "users/logout", 200, "5").await;
  let r_: GetItemResponse = send_get(&c, "", "GET", &format!("items/{id}?page=1"), 200, "25").await;
  assert!(r.comments.is_empty());
  // todo(testing): compare logged in and logged out responses

  // get initial item score and user karma
  let (_points, _karma) = get_points_karma(&c, id).await;
  send(&c, VotePayload::default(), "POST", "items/vote", 401, "30").await;
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "30a").await;
  send(&c, GetItemResponse::default(), "POST", "items/vote", 422, "31").await;

  let upvote = VotePayload::new(id, VoteState::Upvote);
  let downvote = VotePayload::new(id, VoteState::Downvote);
  let nonevote = VotePayload::new(id, VoteState::None);
  vote(&c, &upvote, id, _points, _karma, 1, "32").await;
  vote(&c, &upvote, id, _points, _karma, 0, "33").await;
  vote(&c, &downvote, id, _points, _karma, -1, "34a").await;
  vote(&c, &downvote, id, _points, _karma, 0, "34b").await;
  vote(&c, &upvote, id, _points, _karma, 1, "34c").await;
  vote(&c, &nonevote, id, _points, _karma, 0, "34d").await;

  // favorite
  send(&c, VotePayload::default(), "POST", "items/favorite", 422, "36").await;
  let fpayload = FavoritePayload::new(id, FavoriteStateEnum::Favorite);
  favorite(&c, &fpayload, id, "37a", FavoriteStateEnum::Favorite).await;
  // favorite(&c, &fpayload, id, "37b", FavoriteStateEnum::None).await;

  // get_edit_item_page_data
  send(&c, "", "GET", &format!("items/get-edit-item-page-data/{id}"), 200, "38").await;
  send(&c, "", "GET", &format!("items/get-edit-item-page-data/{bob_item_id}"), 403, "38a").await;
  send(&c, CredentialsPayload::default(), "POST", "users/logout", 200, "38lo").await;
  send(&c, "", "GET", &format!("items/get-edit-item-page-data/{id}"), 401, "38a").await;
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "38li").await;

  // edit item
  let edit = EditItemPayload::new(id, "new title", "new text", ItemCategory::Paper, ItemType::News);
  send(&c, edit, "PUT", "items/edit-item", 422, "39").await;
  let edit = EditItemPayload::new(
    id,
    "new title",
    "new text text text",
    ItemCategory::Paper,
    ItemType::News,
  );
  send(&c, edit, "PUT", "items/edit-item", 200, "39a").await;
  let item =
    send_get::<GetItemResponse>(&c, "", "GET", &format!("items/{id}?page=1"), 200, "40").await.item;
  assert_eq!(item.title, "new title".into());
  assert_eq!(item.text.unwrap(), "new text text text".into());
  assert_eq!(item.item_category, ItemCategory::Paper);
  assert_eq!(item.item_type, ItemType::News);
  let edit = EditItemPayload::new(
    bob_item_id,
    "new title",
    "new text text text",
    ItemCategory::Paper,
    ItemType::News,
  );
  send(&c, edit.clone(), "PUT", "items/edit-item", 403, "41").await;

  // delete
  send(&c, "", "DELETE", &format!("items/delete-item/{id}"), 200, "100").await;
  send(&c, "", "DELETE", &format!("items/delete-item/{id}"), 404, "100a").await;
  send(&c, "", "GET", &format!("items/{id}?page=1"), 404, "100b").await;
}

async fn favorite(
  c: &Client,
  favorite: &FavoritePayload,
  id: Uuid,
  tag: &str,
  expect: FavoriteStateEnum,
) {
  let favorite =
    send_get::<FavoriteStateEnum>(c, favorite, "POST", "items/favorite", 200, tag).await;
  assert_eq!(expect, favorite);
}

async fn vote(
  c: &Client,
  vote: &VotePayload,
  id: Uuid,
  _points: i32,
  _karma: i32,
  inc: i32,
  tag: &str,
) {
  let state = send_get::<VoteState>(c, vote.clone(), "POST", "items/vote", 200, tag).await;
  let expected_state = match inc {
    1 => VoteState::Upvote,
    -1 => VoteState::Downvote,
    _ => VoteState::None,
  };
  assert_eq!(state, expected_state);
  let (points, karma) = get_points_karma(c, id).await;
  assert_eq!(points, _points + inc);
  assert_eq!(karma, _karma + inc);
}

async fn get_points_karma(c: &Client, id: Uuid) -> (i32, i32) {
  let points =
    send_get::<GetItemResponse>(c, "", "GET", &format!("items/{id}?page=1"), 200, "get_points")
      .await
      .item
      .points;
  let karma =
    send_get::<GetUserResponse>(c, "", "GET", "users/alice", 200, "get_karma").await.karma;
  (points, karma)
}
