use db::{
  models::{user_favorite::UserFavorite, user_hidden::UserHidden, user_vote::UserVote},
  DbPool,
};

use super::*;
use crate::AuthUserResponseInternal;

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetItemResponse::default, example=GetItemResponse::default)]
#[serde(rename_all = "camelCase")]
pub struct GetItemResponse {
  pub item:             Item,
  pub comments:         Vec<GetItemCommentResponse>, // todo: transform reduce comment
  pub is_more_comments: bool,
  pub authenticated:    Option<GetItemResponseAuthenticated>,
  pub auth_user:        AuthUserResponseInternal,
}

impl GetItemResponse {
  pub fn new(
    item: Item,
    comments: Vec<GetItemCommentResponse>,
    page: usize,
    get_item_response_authenticated: Option<GetItemResponseAuthenticated>,
    session_user: Option<User>,
  ) -> Self {
    let is_more_comments = comments.len() > page * COMMENTS_PER_PAGE;
    let comments = comments.into_iter().map(GetItemCommentResponse::from).collect();
    let auth_user = AuthUserResponseInternal::new(session_user);
    Self {
      item,
      comments,
      is_more_comments,
      authenticated: get_item_response_authenticated,
      auth_user,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetItemResponseAuthenticated::default, example=GetItemResponseAuthenticated::default)]
#[serde(rename_all = "camelCase")]
pub struct GetItemResponseAuthenticated {
  voted_on_by_user:        bool,
  /// backlog: remove unvote expired as extraneous
  unvote_expired:          bool,
  favorited_by_user:       bool,
  hidden_by_user:          bool,
  edit_and_delete_expired: bool,
}

impl GetItemResponseAuthenticated {
  pub async fn new(
    pool: &DbPool,
    item: &Item,
    vote: &Option<UserVote>,
    favorite: &Option<UserFavorite>,
    hidden: &Option<UserHidden>,
    user: &User,
  ) -> Self {
    let edit_and_delete_expired = item.username != user.username || !item.is_editable(pool);
    Self {
      voted_on_by_user: vote.is_some(),
      unvote_expired: false,
      favorited_by_user: favorite.is_some(),
      hidden_by_user: hidden.is_some(),
      edit_and_delete_expired,
    }
  }
}

// todo(getitemresponsecomment)
#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetItemCommentResponse::default, example=GetItemCommentResponse::default)]
#[serde(rename_all = "camelCase")]
pub struct GetItemCommentResponse {
  comment:                 Comment,
  edit_and_delete_expired: bool,
  // unvote_expired:           bool, - feature removed
  vote_state:              VoteState,
}

impl GetItemCommentResponse {
  pub fn new(comment: Comment, user_comment_votes: &[UserVote]) -> ApiResult<Self> {
    let edit_and_delete_expired = !comment.is_editable();
    let vote_state = user_comment_votes
      .iter()
      .find(|v| v.content_id == comment.id)
      .ok_or(ApiError::OtherISE("Comment vote not found".to_string()))?
      .vote_state;

    // backlog: dropped recursively updating comment children

    Ok(Self { comment, edit_and_delete_expired, vote_state })
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetEditItemResponse::default, example=GetEditItemResponse::default)]
#[serde(rename_all = "camelCase")]
pub struct GetEditItemResponse {
  pub id:               Uuid,
  pub username:         Username,
  pub title:            Title,
  /// news, show, ask
  pub item_type:        ItemType,
  pub url:              Option<Url>,
  pub domain:           Option<Domain>,
  pub text:             Option<Text>,
  /// how many upvotes
  pub points:           i32,
  /// internal algorithmic score to sort items on home page by popularity
  pub score:            i32,
  /// tweet, blog, paper, other
  pub item_category:    ItemCategory,
  pub created:          Timestamp,
  pub dead:             bool,
  /// unique to get-edit item page
  pub text_for_editing: Option<Text>,
}
impl From<Item> for GetEditItemResponse {
  fn from(item: Item) -> Self {
    // backlog(sanitize) - item text
    let text_for_editing = item.text.clone();
    Self {
      id: item.id,
      username: item.username,
      title: item.title,
      item_type: item.item_type,
      url: item.url,
      domain: item.domain,
      text: item.text,
      points: item.points,
      score: item.score,
      item_category: item.item_category,
      created: item.created,
      dead: item.dead,
      text_for_editing,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetItemsPageResponse::default, example=GetItemsPageResponse::default)]
#[serde(rename_all = "camelCase")]
pub struct GetItemsPageResponse {
  /// The items for this page
  // todo: should these items be transformed?
  items: Vec<Item>,
  /// whether there are more items after the page returned
  is_more: bool,
  /// total number of items matching query
  count:   usize,
}
impl GetItemsPageResponse {
  //   // isMore:
  //   totalItemCount >
  //   (page - 1) * config.itemsPerPage + config.itemsPerPage
  //     ? true
  //     : false,
  // };
  pub fn new(items: Vec<Item>, count: usize, page: Page) -> Self {
    let is_more = count > page.page as usize * COMMENTS_PER_PAGE;
    Self { items, is_more, count }
  }
}
