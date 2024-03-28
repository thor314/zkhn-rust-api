use argon2::{
  password_hash::{rand_core::OsRng, SaltString},
  Argon2, PasswordHasher, PasswordVerifier,
};
use tracing::debug;

use crate::{models::user::User, DbError, DbResult, Password, PasswordHash};

pub async fn hash_password_argon(password: &Password) -> DbResult<PasswordHash> {
  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();
  debug!("hashing password");
  let instant = std::time::Instant::now();
  let password_hash = argon2.hash_password(password.0.as_bytes(), &salt)?.to_string();
  let elapsed = instant.elapsed();
  debug!("hashing password, time elapsed: {:?}", elapsed);
  Ok(PasswordHash(password_hash))
}

/// verify the provided password against the user's stored password hash.
/// Convenience wrapper of `verify_password`.
/// Returns Ok(true) if the password matches, Ok(false) if it does not.
/// Returns an error if the stored password hash is invalid.
pub fn verify_user_password(user: &User, other_password: &Password) -> DbResult<bool> {
  verify_password(&user.password_hash, other_password)
}

/// verify the provided password against the user's stored password hash
/// Returns Ok(true) if the password matches, Ok(false) if it does not.
/// Returns an error if the stored password hash is invalid.
pub fn verify_password(pw_hash: &PasswordHash, other_password: &Password) -> DbResult<bool> {
  let parsed_hash = argon2::password_hash::PasswordHash::new(&pw_hash.0)?;
  let argon2 = Argon2::default();
  match argon2.verify_password(other_password.0.as_bytes(), &parsed_hash) {
    Ok(_) => Ok(true),
    Err(_) => Ok(false),
  }
}
