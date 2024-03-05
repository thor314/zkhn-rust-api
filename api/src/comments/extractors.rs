use uuid::Uuid;



pub struct CommentExtractor {
  pub by: String,
  pub parent_item_id: Uuid,
  pub parent_comment_id: Option<Uuid>,
  pub text: String,
  pub dead: bool,
}