use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Debug, PartialEq, Validate, Serialize, Deserialize, Eq)]
pub struct PackageManager(
  #[validate(pattern = r#"(npm|pnpm|yarn|bun)@\d+\.\d+\.\d+(-.+)?"#)] pub String,
);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_pass_validate_package_manager() {
    let package_manager = PackageManager("npm@1.0.0".to_string());
    assert!(package_manager.validate().is_ok());
  }

  #[test]
  fn should_fail_when_package_manager_is_invalid() {
    let package_manager = PackageManager("invalid".to_string());
    assert!(package_manager.validate().is_err());
  }
}
