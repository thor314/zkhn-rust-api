use db::models::user_vote::UserVote;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum VoteState {
  Upvoted,
  Downvoted,
  None,
}

impl From<UserVote> for VoteState {
  fn from(vote: UserVote) -> Self {
    if vote.upvote {
      VoteState::Upvoted
    } else if vote.downvote {
      VoteState::Downvoted
    } else {
      VoteState::None
    }
  }
}
