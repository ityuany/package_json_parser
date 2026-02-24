use crate::ext::Validator;
use jsonc_parser::ast::ObjectProp;
use serde::de::{SeqAccess, Visitor, value::SeqAccessDeserializer};
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

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string or an array of strings for keywords")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Keywords::String(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Keywords::String(value))
      }

      fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
      where
        S: SeqAccess<'de>,
      {
        let values = Vec::<String>::deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(Keywords::Array(values))
      }
    }

    deserializer.deserialize_any(KeywordsVisitor)
  }
}

impl Validator for Keywords {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_deserialize_keywords_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"keywords":["rust","parser"]}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_keywords_when_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"keywords":123}"#);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert!(parsed.validate().is_err());
  }

  #[test]
  fn should_fail_deserialize_keywords_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
