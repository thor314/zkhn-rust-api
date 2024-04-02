use db::{models::item::Item, AuthToken, Timestamp};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

// // success: true,
// // item: item,
// // comments: comments,
// // isMoreComments:
// //   totalNumOfComments >
// //   (page - 1) * config.commentsPerPage + config.commentsPerPage
// //     ? true
// //     : false,
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// #[schema(default = ItemResponse::default, example=ItemResponse::default)]
// pub struct ItemResponse {
//   // todo(refactor): success is redundant
//   pub success: bool,
//   // item:
//   // pub username: Itemname,
//   // pub auth_token: AuthToken,
//   // pub auth_token_expiration_timestamp: Timestamp,
// }

// impl Default for ItemResponse {
//   fn default() -> Self { Self {} }
// }

// impl ItemResponse {
//   pub(crate) fn new(user: Item, auth_token: AuthToken, auth_token_expiration: Timestamp) -> Self
// {     Self {}
//   }
// }

// impl From<Item> for ItemResponse {
//   fn from(user: Item) -> Self { Self {} }
// }
