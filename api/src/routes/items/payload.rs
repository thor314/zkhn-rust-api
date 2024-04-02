use db::{models::item::Item, Username};
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::ApiResult;

/// Itemname, password, and optionally email, and about.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[schema(default = ItemPayload::default, example=ItemPayload::default)]
pub struct ItemPayload {
  #[garde(dive)]
  pub username: Username,
  // #[garde(dive)]
  // pub email:    Option<Email>,
  // #[garde(dive)]
  //   pub about:    Option<About>,
}

impl Default for ItemPayload {
  fn default() -> Self {
    Self {
      username: Username("alice".to_string()),
      // password: Password("password".to_string()),
      // email:    None,
      // about:    None,
    }
  }
}

impl ItemPayload {
  pub async fn into_item(self) -> Item {
    // let password_hash = self.password.hash_argon().await.unwrap();
    // Item::new(self.username, password_hash, self.email, self.about)
    todo!()
  }

  /// convenience method for testing
  pub fn new(
    username: &str,
    // password: &str,
    // email: Option<&str>,
    // about: Option<&str>,
  ) -> ApiResult<Self> {
    let username = Username(username.to_string());
    // let password = Password(password.to_string());
    // let email = email.map(|s| Email(s.to_string()));
    // let about = about.map(|s| About(s.to_string()));
    let payload = Self { username };
    payload.validate(&())?;
    Ok(payload)
  }
}
