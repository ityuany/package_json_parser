use crate::ext::Validator;
use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use rustc_hash::FxHashMap;
use serde::de::{MapAccess, Visitor, value::MapAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Clone, Deref, DerefMut)]
pub struct OptionalDependencies(FxHashMap<String, String>);

impl<'de> Deserialize<'de> for OptionalDependencies {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct OptionalDependenciesVisitor;

    impl<'de> Visitor<'de> for OptionalDependenciesVisitor {
      type Value = OptionalDependencies;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object map for optionalDependencies")
      }

      fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        let value = FxHashMap::<String, String>::deserialize(MapAccessDeserializer::new(map))?;
        Ok(OptionalDependencies(value))
      }
    }

    deserializer.deserialize_any(OptionalDependenciesVisitor)
  }
}

impl Validator for OptionalDependencies {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
