use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone, Deref, DerefMut)]
pub struct Private(bool);

impl<'de> Deserialize<'de> for Private {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct PrivateVisitor;

    impl<'de> Visitor<'de> for PrivateVisitor {
      type Value = Private;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a boolean for private")
      }

      fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Private(value))
      }
    }

    deserializer.deserialize_any(PrivateVisitor)
  }
}

impl Validator for Private {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
