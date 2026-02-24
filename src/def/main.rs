use jsonc_parser::ast::ObjectProp;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone)]
pub enum Main {
  Bool(bool),
  Str(String),
}

impl<'de> Deserialize<'de> for Main {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct MainVisitor;

    impl<'de> Visitor<'de> for MainVisitor {
      type Value = Main;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a boolean or a string for main")
      }

      fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Main::Bool(value))
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Main::Str(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Main::Str(value))
      }
    }

    deserializer.deserialize_any(MainVisitor)
  }
}

impl Validator for Main {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  const FIELD: &str = "main";

  fn parse_field(value: &str) -> miette::Result<PackageJsonParser> {
    PackageJsonParser::parse_str(&format!(r#"{{"{FIELD}":{value}}}"#))
  }

  #[test]
  fn should_deserialize_main_successfully() {
    let parsed = parse_field(r#""index.js""#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_main_when_type_is_invalid() {
    let parsed = parse_field("123");
    assert!(parsed.is_err());
  }

  #[test]
  fn should_fail_deserialize_main_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
