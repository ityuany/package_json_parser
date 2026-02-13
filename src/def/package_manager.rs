use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Clone, Deref, DerefMut)]
pub struct PackageManager(String);

impl Validator for PackageManager {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<crate::validation::RuleViolation> {
    let regex = lazy_regex::regex_is_match!(r#"(npm|pnpm|yarn|bun)@\d+\.\d+\.\d+(-.+)?"#, &self);

    if regex {
      return vec![];
    }

    vec![validation_error(
      "Invalid package manager",
      Some("invalid_package_manager"),
      "Please provide a valid package manager",
      value_range(prop, &[]),
      "",
    )]
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_pass_validate_package_manager() {
    let jsones = [
      r#"{"packageManager": "npm@1.0.0"}"#,
      r#"{"packageManager": "pnpm@1.0.0"}"#,
      r#"{"packageManager": "yarn@1.0.0"}"#,
      r#"{"packageManager": "bun@1.0.0"}"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let report = res.validate().unwrap();
      assert!(!report.has_errors());
      let package_manager = res.get_package_manager();
      assert!(package_manager.value.is_some());
      assert!(!package_manager.has_errors());
    }
  }

  #[test]
  fn should_fail_validate_package_manager() {
    let jsones = [r#"{"packageManager": "invalid"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let report = res.validate().unwrap();
      assert!(report.has_errors());
    }
  }
}
