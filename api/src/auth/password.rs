use argon2::{
  password_hash::{rand_core::OsRng, SaltString},
  Argon2, PasswordHasher, PasswordVerifier,
};
use db::{Password, PasswordHash};
use tokio::task::block_in_place;
use tracing::debug;

use crate::{ApiError, ApiResult};

pub trait PasswordExt {
  fn hash(&self) -> PasswordHash;
  fn hash_and_verify(&self, other: &PasswordHash) -> ApiResult<PasswordHash>;
}

impl PasswordExt for Password {
  fn hash_and_verify(&self, other: &PasswordHash) -> ApiResult<PasswordHash> {
    let hash = self.hash();
    let parsed_hash = argon2::password_hash::PasswordHash::new(&hash.0).unwrap();

    if Argon2::default().verify_password(other.0.as_bytes(), &parsed_hash).is_ok() {
      Ok(hash)
    } else {
      Err(ApiError::Unauthorized("password mismatch".to_string()))
    }
  }

  fn hash(&self) -> PasswordHash {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    debug!("hashing password");
    let instant = std::time::Instant::now();
    let password_hash =
      block_in_place(|| argon2.hash_password(self.0.as_bytes(), &salt).unwrap().to_string());
    let elapsed = instant.elapsed();
    debug!("hashing password, time elapsed: {:?}", elapsed);
    PasswordHash(password_hash)
  }
}
