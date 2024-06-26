use anyhow::Context;
use tracing_subscriber::filter::EnvFilter;

use crate::error::ServerError;

/// Set up crate logging and environment variables.
pub(crate) fn setup(secret_store: &shuttle_runtime::SecretStore) -> Result<(), ServerError> {
  let filter = EnvFilter::from_default_env()
    // .add_directive(LevelFilter::DEBUG.into())
    .add_directive("sqlx=info".parse().unwrap())
    // .add_directive("tower_http=debug".parse().unwrap())
    // .add_directive("tower_sessions=debug".parse().unwrap())
    // .add_directive("axum_login=debug".parse().unwrap())
    .add_directive("axum_login=info".parse().unwrap())
    .add_directive("h2=info".parse().unwrap())
    .add_directive("api=debug".parse().unwrap())
    .add_directive("db=debug".parse().unwrap())
    .add_directive("server=info".parse().unwrap());
  tracing_subscriber::fmt().with_env_filter(filter).init();
  secret_store.get("DOTENV_OK").context("failed to get secrets")?;

  Ok(())
}
