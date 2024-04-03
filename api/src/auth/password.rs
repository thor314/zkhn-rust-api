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
  async fn hash_and_verify(&self, other: &PasswordHash) -> ApiResult<PasswordHash>;
}

impl PasswordExt for Password {
  async fn hash_and_verify(&self, other: &PasswordHash) -> ApiResult<PasswordHash> {
    let hash = self.hash().await;
    let parsed_hash = argon2::password_hash::PasswordHash::new(&hash.0).unwrap();

    if Argon2::default().verify_password(other.0.as_bytes(), &parsed_hash).is_ok() {
      Ok(hash)
    } else {
      Err(ApiError::Unauthorized("password mismatch".to_string()))
    }
  }

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
    trace!("hashing password, time elapsed: {:?}", elapsed);
    PasswordHash(password_hash)
  }
}
