use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone, Deref, DerefMut)]
pub struct Private(bool);

impl<'de> Deserialize<'de> for Private {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct PrivateVisitor;

    impl<'de> Visitor<'de> for PrivateVisitor {
      type Value = Private;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a boolean for private")
      }

      fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Private(value))
      }
    }

    deserializer.deserialize_any(PrivateVisitor)
  }
}

impl Validator for Private {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  const FIELD: &str = "private";

  fn parse_field(value: &str) -> miette::Result<PackageJsonParser> {
    PackageJsonParser::parse_str(&format!(r#"{{"{FIELD}":{value}}}"#))
  }

  #[test]
  fn should_deserialize_private_successfully() {
    let parsed = parse_field("true");
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_private_when_type_is_invalid() {
    let parsed = parse_field(r#""true""#);
    assert!(parsed.is_err());
  }

  #[test]
  fn should_fail_deserialize_private_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
