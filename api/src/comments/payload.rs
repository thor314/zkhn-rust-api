use db::models::comment::Comment;
use serde::Serialize;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug, Serialize, Clone)]
pub struct CommentPayload {
  pub username:          String,
  pub parent_item_id:    Uuid,
  pub parent_comment_id: Option<Uuid>,
  pub text:              String,
  pub dead:              bool,
}

impl TryFrom<CommentPayload> for Comment {
  type Error = anyhow::Error;

  fn try_from(value: CommentPayload) -> Result<Self, Self::Error> {
    // todo validate fields and sanitize text
    todo!()
  }
}
