use super::*;

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[schema(default = GetCommentResponse::default, example=GetCommentResponse::default)]
#[serde(rename_all = "camelCase")]
pub struct GetCommentResponse {
  pub comment:             Comment,
  // pub children:           Vec<Comment>,
}

// impl GetCommentResponse {
//   pub fn new(item: Comment, comments: Vec<Comment>, page: usize) -> Self {
//     let is_more_comments = comments.len() > page * COMMENTS_PER_PAGE;
//     Self { item, comments, is_more_comments }
//   }
// }
