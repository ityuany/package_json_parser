use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Eq, PartialEq, Clone, Deref, DerefMut)]
pub struct Types(String);

impl<'de> Deserialize<'de> for Types {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct TypesVisitor;

    impl<'de> Visitor<'de> for TypesVisitor {
      type Value = Types;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string for types")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Types(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Types(value))
      }
    }

    deserializer.deserialize_any(TypesVisitor)
  }
}

impl Validator for Types {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  const FIELD: &str = "types";

  fn parse_field(value: &str) -> miette::Result<PackageJsonParser> {
    PackageJsonParser::parse_str(&format!(r#"{{"{FIELD}":{value}}}"#))
  }

  #[test]
  fn should_deserialize_types_successfully() {
    let parsed = parse_field(r#""index.d.ts""#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_types_when_type_is_invalid() {
    let parsed = parse_field("false");
    assert!(parsed.is_err());
  }

  #[test]
  fn should_fail_deserialize_types_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
