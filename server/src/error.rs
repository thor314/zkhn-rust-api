//! zkhn-rust-api error types
// https://docs.rs/thiserror/latest/thiserror/

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
  #[error(transparent)]
  Db(#[from] db::error::DbError),
  #[error(transparent)]
  Api(#[from] api::error::ApiError),
  // #[error("an unhandled error")]
  // Unhandled,
}
