use super::*;

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetItemResponse::default, example=GetItemResponse::default)]
#[serde(rename_all = "camelCase")]
pub struct GetItemResponse {
  pub item: Item,
  pub comments: Vec<GetItemResponseComment>, // todo: transform reduce comment
  pub is_more_comments: bool,
  pub get_item_response_authenticated: Option<GetItemResponseAuthenticated>,
}

impl GetItemResponse {
  pub fn new(
    item: Item,
    comments: Vec<Comment>,
    page: usize,
    get_item_response_authenticated: Option<GetItemResponseAuthenticated>,
  ) -> Self {
    let is_more_comments = comments.len() > page * COMMENTS_PER_PAGE;
    let comments = comments.into_iter().map(GetItemResponseComment::from).collect();
    Self { item, comments, is_more_comments, get_item_response_authenticated }
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetItemResponseAuthenticated::default, example=GetItemResponseAuthenticated::default)]
#[serde(rename_all = "camelCase")]
pub struct GetItemResponseAuthenticated {
  voted_on_by_user:        bool,
  unvote_expired:          bool,
  favorited_by_user:       bool,
  hidden_by_user:          bool,
  edit_and_delete_expired: bool,
  // user_comment_votes: Vec<CommentVote>,
}

impl GetItemResponseAuthenticated {
  pub fn new(item: &Item) -> Self {
    // Self {
    // voted_on_by_user,
    // unvote_expired,
    // favorited_by_user,
    // hidden_by_user,
    // edit_and_delete_expired,
    // }
    todo!()
  }
}

// todo(getitemresponsecomment)
#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetItemResponseComment::default, example=GetItemResponseComment::default)]
#[serde(rename_all = "camelCase")]
pub struct GetItemResponseComment {
  comment: Comment,
}

impl From<Comment> for GetItemResponseComment {
  fn from(comment: Comment) -> Self { Self { comment } }
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
