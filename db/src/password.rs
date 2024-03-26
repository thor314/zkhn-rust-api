use scrypt::{
  password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
  Scrypt,
};

use crate::{models::user::User, DbError, DbResult, Password, PasswordHash};

/// Hashes the user's password before saving if it is modified or new.
pub fn hash_password(password: &Password) -> DbResult<PasswordHash> {
  let salt = SaltString::generate(&mut OsRng);
  let pw_hash: scrypt::password_hash::PasswordHash =
    Scrypt.hash_password(password.0.as_bytes(), &salt)?;
  Ok(PasswordHash(pw_hash.to_string()))
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
  let parsed_hash = scrypt::password_hash::PasswordHash::new(&pw_hash.0)?;
  match Scrypt.verify_password(other_password.0.as_bytes(), &parsed_hash) {
    Ok(_) => Ok(true),
    Err(_) => Ok(false),
  }
}
