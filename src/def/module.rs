use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone, Deref, DerefMut)]
pub struct Module(String);

impl<'de> Deserialize<'de> for Module {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ModuleVisitor;

    impl<'de> Visitor<'de> for ModuleVisitor {
      type Value = Module;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string for module")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Module(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Module(value))
      }
    }

    deserializer.deserialize_any(ModuleVisitor)
  }
}

impl Validator for Module {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_deserialize_module_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"module":"index.mjs"}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_module_when_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"module":false}"#);
    assert!(parsed.is_err());
  }

  #[test]
  fn should_fail_deserialize_module_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
