use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Eq, PartialEq, Clone, Deref, DerefMut)]
pub struct Typings(String);

impl<'de> Deserialize<'de> for Typings {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct TypingsVisitor;

    impl<'de> Visitor<'de> for TypingsVisitor {
      type Value = Typings;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string for typings")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Typings(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Typings(value))
      }
    }

    deserializer.deserialize_any(TypingsVisitor)
  }
}

impl Validator for Typings {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_deserialize_typings_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"typings":"index.d.ts"}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_typings_when_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"typings":false}"#);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert!(parsed.typings().is_err());
  }

  #[test]
  fn should_fail_deserialize_typings_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
