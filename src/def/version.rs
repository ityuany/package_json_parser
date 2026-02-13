use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Deref, DerefMut)]
pub struct Version(String);

impl Validator for Version {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<crate::validation::RuleViolation> {
    let regex = lazy_regex::regex_is_match!(
      r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$",
      &self
    );

    if regex {
      return vec![];
    }

    vec![validation_error(
      "Package version does not match required pattern",
      Some("E0001"),
      r"Expected pattern: ^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$",
      value_range(prop, &[]),
      "",
    )]
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_pass_validate_version() {
    let jsones = [
      r#"{"version": "1.0.0"}"#,
      r#"{"version": "1.0.0-alpha.1"}"#,
      r#"{"version": "1.0.0+build.1"}"#,
      r#"{"version": "1.0.0-alpha.1+build.1"}"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let report = res.validate().unwrap();
      assert!(!report.has_errors());
      let version = res.get_version();
      assert!(version.value.is_some());
      assert!(!version.has_errors());
    }
  }

  #[test]
  fn should_fail_validate_version() {
    let jsones = [r#"{"version": "hello"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let report = res.validate().unwrap();
      assert!(report.has_errors());
    }
  }
}
