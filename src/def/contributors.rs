use crate::def::Person;
use jsonc_parser::ast::ObjectProp;
use serde::de::{SeqAccess, Visitor, value::SeqAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub struct Contributors(pub Vec<Person>);

impl<'de> Deserialize<'de> for Contributors {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ContributorsVisitor;

    impl<'de> Visitor<'de> for ContributorsVisitor {
      type Value = Contributors;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an array of contributors")
      }

      fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
      where
        S: SeqAccess<'de>,
      {
        let value = Vec::<Person>::deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(Contributors(value))
      }
    }

    deserializer.deserialize_any(ContributorsVisitor)
  }
}

impl Validator for Contributors {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    // todo!();
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_deserialize_contributors_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"contributors":["alice"]}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_contributors_when_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"contributors":{"name":"alice"}}"#);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert!(parsed.validate().is_err());
  }

  #[test]
  fn should_fail_deserialize_contributors_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
