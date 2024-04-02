mod payload;
mod response;
#[cfg(test)] mod test;

use anyhow::anyhow;
use axum::{
  debug_handler,
  extract::{Path, State},
  http::StatusCode,
  routing, Json, Router,
};
use db::{
  models::{item::Item, user::User},
  password::verify_user_password,
  AuthToken, DbError, Username,
};
use garde::Validate;
pub use payload::*;
pub use response::*;
use tracing::{debug, info, warn};

use super::SharedState;
use crate::{
  // auth::{self, assert_authenticated},
  error::ApiError,
  ApiResult,
};

/// Router to be mounted at "/items"
pub fn items_router(state: SharedState) -> Router {
  Router::new()
    .route("/:id", routing::get(get::get_item))
    .route("/", routing::post(post::create_item))
    .with_state(state)
}

pub(super) mod get {
  use super::*;

  #[utoipa::path(
      get,
      path = "/items/{id}",
      params( ("username" = String, Path, example = "alice") ),
      responses(
        // todo(auth) auth error
        // (status = 401, description = "Unauthorized"),
        (status = 422, description = "Invalid Payload"),
        (status = 422, description = "Invalid username"),
        (status = 500, description = "Database Error"),
        (status = 404, description = "User not found"),
        (status = 200, description = "Success", body = User),// todo(define reduced UserResponse body)
      ),
  )]
  /// Get user.
  ///
  /// If `username` exists, return the User. Otherwise, return NotFound.
  ///
  /// todo(auth): currently, we return the whole user. When auth is implemented, we will want to
  /// return different user data, per the caller's auth.
  ///
  /// ref get_public: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L223
  /// ref get_private: https://github.com/thor314/zkhn/blob/main/rest-api/routes/users/api.js#L244
  pub async fn get_item(
    State(state): State<SharedState>,
    Path(username): Path<Username>,
    // auth_session: AuthSession,  // todo(auth)
  ) -> ApiResult<Json<Item>> {
    debug!("get_item called with username: {username}");
    // let pool = &state.pool;
    // username.validate(&())?;
    // let user = db::queries::users::get_user(pool, &username)
    //   .await?
    //   .ok_or(ApiError::DbEntryNotFound("that user does not exist".to_string()))?;
    // // todo(auth): currently, we return the whole user.
    // // When auth is implemented, we will want to return different user data, per the caller's
    // auth. info!("found user: {user:?}");
    // Ok(Json(user))
    todo!()
  }
}
pub(super) mod post {
  use db::models::item::Item;

  use super::*;
  use crate::auth::AuthSession;

  #[utoipa::path(
      post,
      path = "/items",
      request_body = ItemPayload,
      params( ("username" = String, Path, example = "alice") ),
      responses(
        // todo(auth) auth error
        (status = 401, description = "Unauthorized"),
        (status = 422, description = "Invalid Payload"),
        (status = 500, description = "Database Error"),
        (status = 409, description = "Duplication Conflict"),
        (status = 200, description = "Success", body = ItemResponse), // todo: item response
      ),
  )]
  pub async fn create_item(
    State(state): State<SharedState>,
    auth_session: AuthSession,
    Json(payload): Json<ItemPayload>,
  ) -> ApiResult<Json<Item>> {
    debug!(
      "create_item called with payload: {payload:?} by: {}",
      auth_session.user.as_ref().map(|u| &u.0.username.0).unwrap_or(&"".into())
    );
    // todo(error handling): error passing like this should probably be a defined method for DRY
    let user = auth_session
      .user
      .as_ref()
      .clone()
      .ok_or(ApiError::Unauthorized("must be logged in".to_string()))?;
    if user.0.username != payload.username {
      return Err(ApiError::Unauthorized("must be logged in as the user".to_string()));
    };
    payload.validate(&())?;
    let item: Item = payload.into_item().await;
    db::queries::items::create_item(&state.pool, &item).await?;

    info!("created item: {item:?}");
    Ok(Json(item))
  }
  // database.          Process for the title:
  //          - trim extra spaces from the beginning and end of the string.
  //          - run the string through the XSS NPM package.
  //          Process for the URL:
  //          - trim extra spaces from the beginning and end of the string.
  //          - run the string through the XSS NPM package.
  //          Process for the text:
  //          - trim extra spaces from the beginning and end of the string.
  //          - remove all HTML tags from the string.
  //          - transform all asterisk (*) encapsulated text into <i></i> HTML elements.
  //          - transform all the URLs in the string into <a href=""> HTML elements.
  //          - run the string through the XSS NPM package.
  // Step 3 - Save the new item to the database.
  // Step 4 - In the database, increment the author's karma count by a value of 1.
  // Step 5 - Send a success response back to the website.
  // submitNewItem: async (title, url, text, category, authUser) => {
  // const isValidUrl = utils.isValidUrl(url);
  //
  // if (url && !isValidUrl) {
  // throw { invalidUrlError: true };
  // }
  //
  // filter content
  // title = title.trim();
  // title = xss(title);
  //
  // url = url.trim();
  // url = xss(url);
  //
  // if (text) {
  // text = text.trim();
  // text = text.replace(/<[^>]+>/g, "");
  // text = text.replace(/\*([^*]+)\*/g, "<i>$1</i>");
  // text = linkifyUrls(text);
  // text = xss(text);
  // }
  //
  // const domain = url ? utils.getDomainFromUrl(url) : "";
  // const itemType = utils.getItemType(title, url, text);
  //
  // submit new post/item
  // const newItem = new ItemModel({
  // id: utils.generateUniqueId(12),
  // by: authUser.username,
  // title: title,
  // type: itemType,
  // url: url,
  // domain: domain,
  // text: text,
  // category: category,
  // created: moment().unix(),
  // dead: authUser.shadowBanned ? true : false,
  // });
  //
  // const newItemDoc = await newItem.save();
  //
  // await UserModel.findOneAndUpdate(
  // { username: authUser.username },
  // { $inc: { karma: 1 } },
  // ).exec();
  //
  // if (!authUser.shadowBanned) {
  // await searchApi.addNewItem(newItemDoc);
  // }
  //
  // return { success: true };
  // },
}
pub(super) mod put {
  use super::*;
}
pub(super) mod delete {
  use super::*;
}
