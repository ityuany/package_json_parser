use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use rustc_hash::FxHashMap;
use serde::de::{MapAccess, Visitor, value::MapAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone, Deref, DerefMut)]
pub struct Scripts(FxHashMap<String, String>);

impl<'de> Deserialize<'de> for Scripts {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ScriptsVisitor;

    impl<'de> Visitor<'de> for ScriptsVisitor {
      type Value = Scripts;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object map for scripts")
      }

      fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        let value = FxHashMap::<String, String>::deserialize(MapAccessDeserializer::new(map))?;
        Ok(Scripts(value))
      }
    }

    deserializer.deserialize_any(ScriptsVisitor)
  }
}

impl Validator for Scripts {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_deserialize_scripts_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"scripts":{ "test": "cargo test" }}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_scripts_when_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"scripts":"test"}"#);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert!(parsed.validate().is_err());
  }

  #[test]
  fn should_fail_deserialize_scripts_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
