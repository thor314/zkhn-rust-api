#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use api::*;
use db::models::{item::Item, user_vote::VoteState};
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
  // todo: compare logged in and logged out responses

  // get item score and user karma
  let (_points, _karma) = get_points_karma(&c, id).await;

  // unauthorized user 401
  send(&c, VotePayload::default(), "POST", "items/vote", 401, "30").await;
  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "30a").await;
  // bad payload 422
  send(&c, GetItemResponse::default(), "POST", "items/vote", 422, "31").await;
  // normal upvote 200
  let upvote = VotePayload::new(id, VoteState::Upvote);
  send(&c, upvote.clone(), "POST", "items/vote", 200, "32").await;
  let (points, karma) = get_points_karma(&c, id).await;
  assert_eq!(points, _points + 1);
  assert_eq!(karma, _karma + 1);
  // duplicate upvote 409
  send(&c, upvote.clone(), "POST", "items/vote", 409, "33").await;

  // vote score and karma checks
  let downvote = VotePayload::new(id, VoteState::Downvote);
  send(&c, downvote.clone(), "POST", "items/vote", 200, "34a").await;
  let (points, karma) = get_points_karma(&c, id).await;
  assert_eq!(points, _points - 1);
  assert_eq!(karma, _karma - 1);
  send(&c, upvote.clone(), "POST", "items/vote", 200, "34b").await;
  let (points, karma) = get_points_karma(&c, id).await;
  assert_eq!(points, _points + 1);
  assert_eq!(karma, _karma + 1);
  let unvote = VotePayload::new(id, VoteState::None);
  send(&c, unvote.clone(), "POST", "items/vote", 200, "34c").await;
  let (points, karma) = get_points_karma(&c, id).await;
  assert_eq!(points, _points);
  assert_eq!(karma, _karma);

  // bad payload: 400
  send(&c, VotePayload::default(), "POST", "items/favorite", 400, "36").await;
  // normal favorites and unfavorites: 200; duplicate favorite: 409
  let favorite = FavoritePayload::new(id, FavoritePayloadEnum::Favorite);
  send(&c, favorite.clone(), "POST", "items/favorite", 200, "35").await;
  send(&c, favorite.clone(), "POST", "items/favorite", 409, "35a").await;
  let unfavorite = FavoritePayload::new(id, FavoritePayloadEnum::Unfavorite);
  send(&c, unfavorite.clone(), "POST", "items/favorite", 200, "35b").await;
  send(&c, unfavorite.clone(), "POST", "items/favorite", 409, "35c").await;
  send(&c, favorite.clone(), "POST", "items/favorite", 200, "35d").await;
  // logged out: 401
  send(&c, CredentialsPayload::default(), "POST", "users/logout", 200, "5").await;
  send(&c, unfavorite.clone(), "POST", "items/favorite", 401, "36").await;

  send(&c, CredentialsPayload::default(), "POST", "users/login", 200, "37").await;

  let hide = HiddenPayload::new(id, HiddenPayloadEnum::Hidden);
  send(&c, hide.clone(), "POST", "items/hide", 200, "38").await;
  send(&c, hide, "POST", "items/hide", 400, "38a").await;
  let unhide = HiddenPayload::new(id, HiddenPayloadEnum::UnHidden);
  send(&c, unhide.clone(), "POST", "items/hide", 200, "38b").await;
  send(&c, unhide, "POST", "items/hide", 400, "38c").await;

  // get edit item
  // edit item
  // get delete item
  // delete item
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
