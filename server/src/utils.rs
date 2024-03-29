use anyhow::Context;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

use crate::error::ServerError;

/// Set up crate logging and environment variables.
pub(crate) fn setup(secret_store: &shuttle_runtime::SecretStore) -> Result<(), ServerError> {
  let default = "axum_login=debug,tower_sessions=debug,sqlx=warn,tower_http=debug,server=debug, \
                 db=debug,api=debug"
    .parse()
    .unwrap();
  let filter = EnvFilter::builder().with_default_directive(default).from_env_lossy();
  tracing_subscriber::fmt().with_env_filter(filter).init();
  secret_store.get("DOTENV_OK").context("failed to get secrets")?;
  Ok(())
}