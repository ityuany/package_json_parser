use jsonc_parser::ast::ObjectProp;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone)]
pub enum Readme {
  String(String),
  Object(ReadmeContent),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReadmeContent {
  pub r#type: String,
  pub value: String,
}

impl<'de> Deserialize<'de> for Readme {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ReadmeVisitor;

    impl<'de> Visitor<'de> for ReadmeVisitor {
      type Value = Readme;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
          .write_str("a string or object `{ \"type\": string, \"value\": string }` for `readme`")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Readme::String(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Readme::String(value))
      }

      fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        let content =
          ReadmeContent::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
        Ok(Readme::Object(content))
      }
    }

    deserializer.deserialize_any(ReadmeVisitor)
  }
}

impl Validator for Readme {
  fn validate(&self, _prop: Option<&ObjectProp>) -> Vec<crate::validation::RuleViolation> {
    vec![]
  }
}
