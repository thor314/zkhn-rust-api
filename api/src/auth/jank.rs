
  use super::*;

  // todo: questionably secure, certainly jank, but for now it's in the tank
  pub fn generate_user_token() -> (AuthToken, Timestamp) {
    let mut rng = rand::thread_rng();
    // generate a 40 char token
    let random_hex_string: String = (0..40)
      .map(|_| rng.sample(rand::distributions::Alphanumeric))
      .map(|c| c as char)
      .filter(|c| c.is_ascii_hexdigit())
      .collect();
    let token = AuthToken(random_hex_string);
    let expiration = crate::utils::default_expiration();

    debug!("generated token: {:?}", token);
    (token, expiration)
  }