use anyhow::{anyhow, Context};
use tracing::trace;
use tracing_subscriber::{
  filter::{EnvFilter, LevelFilter},
  layer::SubscriberExt,
  util::SubscriberInitExt,
};

use crate::error::MyError;
/// Set up crate logging and environment variables.
pub(crate) fn setup(secret_store: &shuttle_secrets::SecretStore) -> Result<(), MyError> {
  let filter =
    EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy();
  tracing_subscriber::fmt().with_env_filter(filter).init();
  secret_store.get("DOTENV_OK").unwrap();
  Ok(())
}
