use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Eq, PartialEq, Clone, Deref, DerefMut)]
pub struct Types(String);

impl<'de> Deserialize<'de> for Types {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct TypesVisitor;

    impl<'de> Visitor<'de> for TypesVisitor {
      type Value = Types;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string for types")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Types(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Types(value))
      }
    }

    deserializer.deserialize_any(TypesVisitor)
  }
}

impl Validator for Types {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
