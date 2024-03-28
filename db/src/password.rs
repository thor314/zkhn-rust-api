use scrypt::{
  password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
  Scrypt,
};
use tracing::debug;

use crate::{models::user::User, DbError, DbResult, Password, PasswordHash};

pub async fn hash_password(password: &Password) -> DbResult<PasswordHash> {
  let salt = SaltString::generate(&mut OsRng);
  let password = password.clone();
  // Move `salt` into the closure
  let pw_hash = tokio::task::spawn_blocking(move || {
    let out = Scrypt.hash_password(password.0.as_bytes(), &salt)?.to_string();
    Ok::<String, scrypt::password_hash::Error>(out)
  })
  .await??;
  Ok(PasswordHash(pw_hash))
}

// /// Hashes the user's password before saving if it is modified or new.
// pub async fn hash_password(password: Password) -> DbResult<PasswordHash> {
//   let salt = SaltString::generate(&mut OsRng);
//   let pw_hash = tokio::task::spawn_blocking(move || {
//     // let password = password.clone();
//     // let salt = salt.clone();
//     Scrypt.hash_password(password.0.as_bytes(), &salt)
//   })
//   .await??;
//   debug!("ok_password");
//   Ok(PasswordHash(pw_hash.to_string()))
// }

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
  let parsed_hash = scrypt::password_hash::PasswordHash::new(&pw_hash.0)?;
  match Scrypt.verify_password(other_password.0.as_bytes(), &parsed_hash) {
    Ok(_) => Ok(true),
    Err(_) => Ok(false),
  }
}
