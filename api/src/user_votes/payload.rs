use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum VoteState {
  Upvoted,
  Downvoted,
  None,
}