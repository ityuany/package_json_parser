use std::collections::HashMap;

use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::{MapAccess, Visitor, value::MapAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone, Deref, DerefMut)]
pub struct Engines(HashMap<String, String>);

impl<'de> Deserialize<'de> for Engines {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct EnginesVisitor;

    impl<'de> Visitor<'de> for EnginesVisitor {
      type Value = Engines;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object map for engines")
      }

      fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        let value = HashMap::<String, String>::deserialize(MapAccessDeserializer::new(map))?;
        Ok(Engines(value))
      }
    }

    deserializer.deserialize_any(EnginesVisitor)
  }
}

impl Validator for Engines {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_deserialize_engines_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"engines":{"node":">=18"}}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_engines_when_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"engines":"node"}"#);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert!(parsed.validate().is_err());
  }

  #[test]
  fn should_fail_deserialize_engines_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
