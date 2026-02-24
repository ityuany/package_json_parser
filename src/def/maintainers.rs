use crate::Person;
use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::{SeqAccess, Visitor, value::SeqAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Eq, PartialEq, Clone, Deref, DerefMut)]
pub struct Maintainers(Vec<Person>);

impl<'de> Deserialize<'de> for Maintainers {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct MaintainersVisitor;

    impl<'de> Visitor<'de> for MaintainersVisitor {
      type Value = Maintainers;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an array of maintainers")
      }

      fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
      where
        S: SeqAccess<'de>,
      {
        let value = Vec::<Person>::deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(Maintainers(value))
      }
    }

    deserializer.deserialize_any(MaintainersVisitor)
  }
}

impl Validator for Maintainers {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
