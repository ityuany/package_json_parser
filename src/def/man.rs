use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::{SeqAccess, Visitor, value::SeqAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone, Deref, DerefMut)]
pub struct Man(Vec<String>);

impl<'de> Deserialize<'de> for Man {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ManVisitor;

    impl<'de> Visitor<'de> for ManVisitor {
      type Value = Man;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an array of man entries")
      }

      fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
      where
        S: SeqAccess<'de>,
      {
        let value = Vec::<String>::deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(Man(value))
      }
    }

    deserializer.deserialize_any(ManVisitor)
  }
}

impl Validator for Man {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  const FIELD: &str = "man";

  fn parse_field(value: &str) -> miette::Result<PackageJsonParser> {
    PackageJsonParser::parse_str(&format!(r#"{{"{FIELD}":{value}}}"#))
  }

  #[test]
  fn should_deserialize_man_successfully() {
    let parsed = parse_field(r#"["man1","man2"]"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_man_when_type_is_invalid() {
    let parsed = parse_field(r#""man1""#);
    assert!(parsed.is_err());
  }

  #[test]
  fn should_fail_deserialize_man_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
