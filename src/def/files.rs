use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::{SeqAccess, Visitor, value::SeqAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Eq, PartialEq, Clone, Deref, DerefMut)]
pub struct Files(Vec<String>);

impl<'de> Deserialize<'de> for Files {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct FilesVisitor;

    impl<'de> Visitor<'de> for FilesVisitor {
      type Value = Files;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an array of files entries")
      }

      fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
      where
        S: SeqAccess<'de>,
      {
        let value = Vec::<String>::deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(Files(value))
      }
    }

    deserializer.deserialize_any(FilesVisitor)
  }
}

impl Validator for Files {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_deserialize_files_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"files":["dist", "README.md"]}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_files_when_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"files":{"a":"b"}}"#);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert!(parsed.files().is_err());
  }

  #[test]
  fn should_fail_deserialize_files_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
