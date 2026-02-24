use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone, Deref, DerefMut)]
pub struct EngineStrict(bool);

impl<'de> Deserialize<'de> for EngineStrict {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct EngineStrictVisitor;

    impl<'de> Visitor<'de> for EngineStrictVisitor {
      type Value = EngineStrict;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a boolean for engineStrict")
      }

      fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(EngineStrict(value))
      }
    }

    deserializer.deserialize_any(EngineStrictVisitor)
  }
}

impl Validator for EngineStrict {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
