use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, PartialEq, Serialize, Clone, Deref, DerefMut)]
pub struct Description(String);

impl<'de> Deserialize<'de> for Description {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct DescriptionVisitor;

    impl<'de> Visitor<'de> for DescriptionVisitor {
      type Value = Description;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string for description")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Description(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Description(value))
      }
    }

    deserializer.deserialize_any(DescriptionVisitor)
  }
}

impl Validator for Description {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_deserialize_description_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"description":"hello"}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_description_when_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"description":123}"#);
    assert!(parsed.is_err());
  }

  #[test]
  fn should_fail_deserialize_description_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
