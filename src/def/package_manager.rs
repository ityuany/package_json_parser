use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Deserializer, Serialize};

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, PartialEq, Serialize, Eq, Clone, Deref, DerefMut)]
pub struct PackageManager(String);

impl<'de> Deserialize<'de> for PackageManager {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    String::deserialize(deserializer).map(Self)
  }
}

impl Validator for PackageManager {
  fn validate(&self, prop: Option<&ObjectProp>) -> miette::Result<()> {
    let regex = lazy_regex::regex_is_match!(r#"(npm|pnpm|yarn|bun)@\d+\.\d+\.\d+(-.+)?"#, &self);

    if regex {
      return Ok(());
    }

    Err(validation_error(
      "Invalid package manager",
      Some("invalid_package_manager"),
      "Please provide a valid package manager",
      value_range(prop, &[]),
      "here",
    ))
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
      let res = res.validate();
      assert!(res.is_ok());
    }
  }

  #[test]
  fn should_fail_validate_package_manager() {
    let jsones = [r#"{"packageManager": "invalid"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_err());
    }
  }
}
