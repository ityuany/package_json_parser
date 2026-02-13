use crate::ext::Validator;
use jsonc_parser::ast::ObjectProp;
use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Clone)]
pub enum Keywords {
  String(String),
  Array(Vec<String>),
}

impl<'de> Deserialize<'de> for Keywords {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct KeywordsVisitor;

    impl<'de> Visitor<'de> for KeywordsVisitor {
      type Value = Keywords;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or array of strings for `keywords`")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Keywords::String(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Keywords::String(value))
      }

      fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
      where
        A: SeqAccess<'de>,
      {
        let items = Vec::<String>::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))?;
        Ok(Keywords::Array(items))
      }
    }

    deserializer.deserialize_any(KeywordsVisitor)
  }
}

impl Validator for Keywords {
  fn validate(&self, _prop: Option<&ObjectProp>) -> Vec<crate::validation::RuleViolation> {
    vec![]
  }
}
