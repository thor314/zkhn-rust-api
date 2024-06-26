use super::*;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, ToSchema, Clone)]
/// Represents a vote cast by a user on an item or comment.
pub struct UserVote {
  pub id:             Ulid,
  /// The username of the user who cast the vote.
  pub username:       Username,
  /// The type of content voted on.
  /// Item, Comment
  pub vote_type:      ItemOrComment,
  /// The ID of the item or comment voted on.
  pub content_id:     Ulid,
  /// The ID of the parent item for votes on comments.
  // backlog: From<Option<String>> is not implemented for Option<Ulid> and can't implement it
  pub parent_item_id: Option<String>,
  pub vote_state:     VoteState,
  /// When the vote was cast.
  pub created:        Timestamp,
}

impl UserVote {
  pub fn new(
    username: Username,
    vote_type: ItemOrComment,
    content_id: Ulid,
    parent_item_id: Option<String>,
    vote_state: VoteState,
  ) -> Self {
    Self {
      id: Ulid::new(),
      username,
      vote_type,
      content_id,
      parent_item_id,
      vote_state,
      created: now(),
    }
  }
}

#[derive(sqlx::Type, Default, PartialEq, Serialize, Deserialize, Debug, Clone, ToSchema, Copy)]
#[sqlx(type_name = "vote_state_enum")]
#[sqlx(rename_all = "camelCase")]
pub enum VoteState {
  #[default]
  Upvote,
  Downvote,
  None,
}
impl From<VoteState> for i32 {
  fn from(v: VoteState) -> Self {
    match v {
      VoteState::Upvote => 1,
      VoteState::Downvote => -1,
      VoteState::None => 0,
    }
  }
}

#[derive(sqlx::Type, PartialEq, Serialize, Deserialize, Debug, Clone, ToSchema)]
#[sqlx(type_name = "item_or_comment_enum")]
#[sqlx(rename_all = "camelCase")]
pub enum ItemOrComment {
  Item,
  Comment,
}
impl Default for ItemOrComment {
  fn default() -> Self { Self::Item }
}
impl fmt::Display for ItemOrComment {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ItemOrComment::Item => write!(f, "item"),
      ItemOrComment::Comment => write!(f, "comment"),
    }
  }
}

// let submitter =
// if increment_value > 0 {
//   sqlx::query!("UPDATE items SET points = points + $1 WHERE id = $2
//   RETURNING username", increment_value, item_id)
//     .execute(&mut *tx)
//     .await?;
// } else if increment_value < 0 {
//   sqlx::query!("UPDATE items SET points = points - $1 WHERE id = $2
//   RETURNING username", increment_value, item_id)
//     .execute(&mut *tx)
//     .await?;
// } else{

// }
