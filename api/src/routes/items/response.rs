use std::collections::HashMap;

use db::{
  models::{user_favorite::UserFavorite, user_vote::UserVote},
  queries::ITEM_PAGE_SIZE,
  DbPool, Ulid,
};

use super::*;
use crate::AuthUserResponseInternal;

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetItemResponse::default, example=GetItemResponse::default)]
#[serde(rename_all = "camelCase")]
pub struct GetItemResponse {
  pub item:          Item,
  pub with_comments: Option<WithCommentsResponse>,
  pub auth_user:     AuthUserResponseInternal,
}

impl GetItemResponse {
  /// - compute whether there are more comments beyond this page
  /// - transform the comments into responses
  /// - the user authentication information
  pub fn new(
    item: Item,
    comments: Vec<Comment>,
    page: usize,
    authenticated_item_data: Option<GetItemResponseAuthenticated>,
    session_user: Option<User>,
    mut user_comment_votes: Option<Vec<UserVote>>,
  ) -> ApiResult<Self> {
    let auth_user = AuthUserResponseInternal::new(session_user);
    let with_comments = match user_comment_votes {
      Some(votes) =>
        Some(WithCommentsResponse::new(comments, page, authenticated_item_data, votes.clone())?),
      None => None,
    };

    Ok(Self { item, with_comments, auth_user })
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetItemResponseAuthenticated::default, example=GetItemResponseAuthenticated::default)]
#[serde(rename_all = "camelCase")]
pub struct GetItemResponseAuthenticated {
  voted_on_by_user:        bool,
  /// note: remove unvote expired as extraneous
  /// note: hidden removed
  // unvote_expired:          bool, // unvote expired removed
  favorited_by_user: bool,
  edit_and_delete_expired: bool,
}

impl GetItemResponseAuthenticated {
  pub async fn new(
    pool: &DbPool,
    item: &Item,
    vote: &Option<UserVote>,
    favorite: &Option<UserFavorite>,
    user: &User,
  ) -> Self {
    let edit_and_delete_expired = item.username != user.username || !item.is_editable();
    Self {
      voted_on_by_user: vote.is_some(),
      favorited_by_user: favorite.is_some(),
      edit_and_delete_expired,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = WithCommentsResponse::default, example=WithCommentsResponse::default)]
#[serde(rename_all = "camelCase")]
pub struct WithCommentsResponse {
  pub comments:                Vec<GetItemCommentResponse>, // todo: transform reduce comment
  pub is_more_comments:        bool,
  pub authenticated_item_data: Option<GetItemResponseAuthenticated>,
}
impl WithCommentsResponse {
  pub fn new(
    comments: Vec<Comment>,
    page: usize,
    authenticated_item_data: Option<GetItemResponseAuthenticated>,
    user_comment_votes: Vec<UserVote>,
  ) -> ApiResult<Self> {
    let is_more_comments = comments.len() > page * COMMENTS_PER_PAGE;
    let comments = comments
      .into_iter()
      .map(|comment| GetItemCommentResponse::new(comment, user_comment_votes.clone()))
      .collect::<ApiResult<Vec<_>>>()?;
    Ok(Self { comments, is_more_comments, authenticated_item_data })
  }
}

// todo(getitemresponsecomment)
#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetItemCommentResponse::default, example=GetItemCommentResponse::default)]
#[serde(rename_all = "camelCase")]
pub struct GetItemCommentResponse {
  comment:                 Comment,
  edit_and_delete_expired: bool,
  vote_state:              VoteState,
}

impl GetItemCommentResponse {
  /// - compute whether the comment is editable
  /// - get the user's vote for this comment
  /// - return the comment, the vote, and whether the comment is editable
  pub fn new(comment: Comment, user_comment_votes: Vec<UserVote>) -> ApiResult<Self> {
    let edit_and_delete_expired = !comment.is_editable();
    let vote_state = user_comment_votes
      .iter()
      .find(|v| v.content_id == comment.id)
      .ok_or(ApiError::OtherISE("Comment vote not found".to_string()))?
      .vote_state;

    Ok(Self { comment, edit_and_delete_expired, vote_state })
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetItemsPageResponse::default, example=GetItemsPageResponse::default)]
#[serde(rename_all = "camelCase")]
pub struct GetItemsPageResponse {
  /// The items for this page
  // todo: should these items be transformed?
  items: Vec<RankedItemResponse>,
  /// whether there are more items after the page returned
  is_more: bool,
  /// total number of items matching query
  count:   usize,
}
impl GetItemsPageResponse {
  pub fn new(
    items: Vec<Item>,
    count: usize,
    page: Page,
    votes: HashMap<Ulid, UserVote>,
    username: Option<&Username>,
  ) -> Self {
    let is_more = count > page.page as usize * ITEM_PAGE_SIZE as usize;
    let items = items
      .into_iter()
      .enumerate()
      .map(|(n, item)| {
        let vote = votes.get(&item.id).cloned();
        RankedItemResponse::new(n, item, vote, username)
      })
      .collect();
    Self { items, is_more, count }
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = RankedItemResponse::default, example=RankedItemResponse::default)]
#[serde(rename_all = "camelCase")]
pub struct RankedItemResponse {
  pub page_rank:               usize,
  pub item:                    Item,
  pub vote:                    Option<UserVote>,
  pub edit_and_delete_expired: bool,
}
impl RankedItemResponse {
  pub fn new(
    page_rank: usize,
    item: Item,
    vote: Option<UserVote>,
    username: Option<&Username>,
  ) -> Self {
    let edit_and_delete_expired =
      item.username != username.cloned().unwrap_or_default() || !item.is_editable();
    Self { page_rank, item, vote, edit_and_delete_expired }
  }
}
