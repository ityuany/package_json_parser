use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::{SeqAccess, Visitor, value::SeqAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone, Deref, DerefMut)]
pub struct Cpu(Vec<String>);

impl<'de> Deserialize<'de> for Cpu {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct CpuVisitor;

    impl<'de> Visitor<'de> for CpuVisitor {
      type Value = Cpu;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an array of cpu entries")
      }

      fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
      where
        S: SeqAccess<'de>,
      {
        let value = Vec::<String>::deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(Cpu(value))
      }
    }

    deserializer.deserialize_any(CpuVisitor)
  }
}

impl Validator for Cpu {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_deserialize_cpu_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"cpu":["x64"]}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_cpu_when_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"cpu":"x64"}"#);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert!(parsed.cpu().is_err());
  }

  #[test]
  fn should_fail_deserialize_cpu_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
