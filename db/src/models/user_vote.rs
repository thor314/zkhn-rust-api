use super::*;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, ToSchema)]
/// Represents a vote cast by a user on an item or comment.
pub struct UserVote {
  /// The username of the user who cast the vote.
  pub username:       Username,
  /// The type of content voted on.
  /// Item, Comment
  pub vote_type:      ItemOrComment,
  /// The ID of the item or comment voted on.
  pub content_id:     Uuid,
  /// The ID of the parent item for votes on comments.
  pub parent_item_id: Option<Uuid>,
  pub vote_state:     VoteState,
  /// When the vote was cast.
  pub created:        Timestamp,
}

impl UserVote {
  pub fn new(
    username: Username,
    vote_type: ItemOrComment,
    content_id: Uuid,
    parent_item_id: Option<Uuid>,
    vote_state: VoteState,
  ) -> Self {
    Self { username, vote_type, content_id, parent_item_id, vote_state, created: now() }
  }
}

#[derive(sqlx::Type, Default, PartialEq, Serialize, Deserialize, Debug, Clone, ToSchema, Copy)]
#[sqlx(type_name = "vote_state_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum VoteState {
  #[default]
  Upvote,
  Downvote,
  None,
}
impl From<VoteState> for i8 {
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
#[sqlx(rename_all = "lowercase")]
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
