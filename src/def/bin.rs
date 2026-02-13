use jsonc_parser::ast::ObjectProp;
use serde::de::{self, MapAccess, Visitor};
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

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or object map `{ \"name\": \"path\" }` for `bin`")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Bin::String(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Bin::String(value))
      }

      fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        let object = HashMap::<String, String>::deserialize(
          serde::de::value::MapAccessDeserializer::new(map),
        )?;
        Ok(Bin::Object(object))
      }
    }

    deserializer.deserialize_any(BinVisitor)
  }
}

impl Validator for Bin {
  fn validate(&self, _bin: Option<&ObjectProp>) -> Vec<crate::validation::RuleViolation> {
    vec![]
  }
}
