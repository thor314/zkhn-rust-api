use argon2::{
  password_hash::{rand_core::OsRng, SaltString},
  Argon2, PasswordHasher, PasswordVerifier,
};
use db::{Password, PasswordHash};
use tokio::task::{block_in_place, spawn_blocking};
use tracing::trace;

use crate::{ApiError, ApiResult};

pub trait PasswordExt {
  async fn hash(&self) -> PasswordHash;
  async fn hash_and_verify(&self, other_hash: &PasswordHash) -> ApiResult<()>;
}

impl PasswordExt for Password {
  /// Hashes the password using argon2 and compares it to the provided hash.
  ///
  /// Ok(())            - Password matches provided hash
  /// Err(Unauthorized) - Password does not match provided hash
  async fn hash_and_verify(&self, other_hash: &PasswordHash) -> ApiResult<()> {
    let password_bytes = self.0.as_bytes(); // Clone the password data
    let parsed_hash = argon2::password_hash::PasswordHash::new(&other_hash.0).unwrap();
    Argon2::default()
      .verify_password(password_bytes, &parsed_hash)
      .map_err(|e| ApiError::Unauthorized(format!("password mismatch: {e}")))?;
    Ok(())
  }

  /// Hashes the password using argon2. 
  async fn hash(&self) -> PasswordHash {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_bytes = self.0.as_bytes().to_owned(); // Clone the password data
    let instant = std::time::Instant::now();
    let password_hash =
      spawn_blocking(move || argon2.hash_password(&password_bytes, &salt).unwrap().to_string())
        .await
        .expect("tokio runtime error");
    let elapsed = instant.elapsed();
    trace!("hashing password, time elapsed: {:?}", elapsed); // ~400ms 
    PasswordHash(password_hash)
  }
}
