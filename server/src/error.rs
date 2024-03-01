//! zkhn-rust-api error types
// https://docs.rs/thiserror/latest/thiserror/

use diesel_async::pooled_connection::deadpool::PoolError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
  #[error("My Io error: {0}")]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
  #[error("deadpool error: {0}")]
  Deadpool(#[from] PoolError),
  #[allow(dead_code)]
  #[error(transparent)]
  Db(#[from] db::error::MyError),
  #[error(transparent)]
  Api(#[from] api::error::ApiError),
  #[error("an unhandled error")]
  Unhandled,
}
