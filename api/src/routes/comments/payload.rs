//! Provided methods:
//! - `create_comment`
//! - `get_comment`
//! - `update_comment_vote`
//! - `update_comment_favorite`
//! - `update_comment_text`
//! - `delete_comment`

use super::*;

// corresponding to `add_new_comment` in API
#[derive(Debug, Deserialize, Validate, Default, Clone, Serialize, ToSchema)]
#[schema(default = CreateItemPayload::default, example=CreateItemPayload::default)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentPayload {
  #[garde(dive)]
  pub username:          Username,
  #[garde(skip)]
  pub parent_item_id:    Uuid,
  #[garde(dive)]
  pub parent_item_title: Title,
  #[garde(skip)]
  pub is_parent:         bool,
  #[garde(skip)]
  pub root_comment_id:   Option<Uuid>,
  #[garde(skip)]
  pub parent_comment_id: Option<Uuid>,
  #[garde(dive)]
  pub text:              CommentText,
  #[garde(skip)]
  pub dead:              bool,
}

impl TryFrom<CreateCommentPayload> for Comment {
  type Error = ApiError;

  fn try_from(payload: CreateCommentPayload) -> ApiResult<Self> {
    payload.validate(&())?;

    let comment = Comment::new(
      payload.username,
      payload.parent_item_id,
      payload.parent_item_title,
      payload.is_parent,
      payload.root_comment_id,
      payload.parent_comment_id,
      payload.text,
      payload.dead,
    );

    Ok(comment)
  }
}

impl CreateCommentPayload {
  /// Assume Comment Payload has already been validated.
  pub fn into_comment(self) -> Comment {
    Comment::new(
      self.username,
      self.parent_item_id,
      self.parent_item_title,
      self.is_parent,
      self.root_comment_id,
      self.parent_comment_id,
      self.text,
      self.dead,
    )
  }
}
