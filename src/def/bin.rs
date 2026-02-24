use jsonc_parser::ast::ObjectProp;
use serde::de::{MapAccess, Visitor, value::MapAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone)]
pub enum Bin {
  String(String),
  Object(HashMap<String, String>),
}

impl<'de> Deserialize<'de> for Bin {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct BinVisitor;

    impl<'de> Visitor<'de> for BinVisitor {
      type Value = Bin;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string or an object for bin")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Bin::String(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Bin::String(value))
      }

      fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        let value = HashMap::<String, String>::deserialize(MapAccessDeserializer::new(map))?;
        Ok(Bin::Object(value))
      }
    }

    deserializer.deserialize_any(BinVisitor)
  }
}

impl Validator for Bin {
  fn validate(&self, _bin: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_deserialize_bin_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"bin":"cli.js"}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_bin_when_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"bin":123}"#);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert!(parsed.bin().is_err());
  }

  #[test]
  fn should_fail_deserialize_bin_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
