use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct Name(
  #[validate(
    pattern = "^(?:(?:@(?:[a-z0-9-*~][a-z0-9-*._~]*)?/[a-z0-9-._~])|[a-z0-9-~])[a-z0-9-._~]*$"
  )]
  pub String,
);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_pass_validate_name() {
    let name = Name("test".to_string());
    assert!(name.validate().is_ok());
  }

  #[test]
  fn should_fail_when_name_is_invalid() {
    let name = Name("tEst".to_string());
    assert!(name.validate().is_err());
  }
}
