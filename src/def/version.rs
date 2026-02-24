use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, PartialEq, Serialize, Clone, Deref, DerefMut)]
pub struct Version(String);

impl<'de> Deserialize<'de> for Version {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct VersionVisitor;

    impl<'de> Visitor<'de> for VersionVisitor {
      type Value = Version;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string for version")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Version(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Version(value))
      }
    }

    deserializer.deserialize_any(VersionVisitor)
  }
}

impl Validator for Version {
  fn validate(&self, prop: Option<&ObjectProp>) -> miette::Result<()> {
    let regex = lazy_regex::regex_is_match!(
      r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$",
      &self
    );

    if regex {
      return Ok(());
    }

    Err(validation_error(
      "Package version does not match required pattern",
      Some("E0001"),
      r"Expected pattern: ^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$",
      value_range(prop, &[]),
      "here",
    ))
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
      let res = res.validate();
      assert!(res.is_ok());
    }
  }

  #[test]
  fn should_fail_validate_version() {
    let jsones = [r#"{"version": "hello"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_err());
    }
  }
}
