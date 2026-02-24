use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, PartialEq, Serialize, Eq, Clone, Deref, DerefMut)]
pub struct PackageManager(String);

impl<'de> Deserialize<'de> for PackageManager {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct PackageManagerVisitor;

    impl<'de> Visitor<'de> for PackageManagerVisitor {
      type Value = PackageManager;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string for packageManager")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(PackageManager(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(PackageManager(value))
      }
    }

    deserializer.deserialize_any(PackageManagerVisitor)
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
