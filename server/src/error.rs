//! zkhn-rust-api error types
// https://docs.rs/thiserror/latest/thiserror/

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
  #[error(transparent)]
  Db(#[from] db::error::DbError),
  #[error(transparent)]
  Api(#[from] api::error::ApiError),

  #[error(transparent)]
  TaskJoin(#[from] tokio::task::JoinError),
  #[error(transparent)]
  Session(#[from] tower_sessions::session_store::Error),
  #[error(transparent)]
  Shuttle(#[from] shuttle_runtime::Error),
}

impl From<ServerError> for shuttle_runtime::Error {
  fn from(e: ServerError) -> Self { shuttle_runtime::Error::Custom(e.into()) }
}
