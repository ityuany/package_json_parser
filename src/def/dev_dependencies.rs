use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use rustc_hash::FxHashMap;
use serde::de::{MapAccess, Visitor, value::MapAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone, Deref, DerefMut, Default)]
pub struct DevDependencies(FxHashMap<String, String>);

impl<'de> Deserialize<'de> for DevDependencies {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct DevDependenciesVisitor;

    impl<'de> Visitor<'de> for DevDependenciesVisitor {
      type Value = DevDependencies;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object map for devDependencies")
      }

      fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        let value = FxHashMap::<String, String>::deserialize(MapAccessDeserializer::new(map))?;
        Ok(DevDependencies(value))
      }
    }

    deserializer.deserialize_any(DevDependenciesVisitor)
  }
}

impl Validator for DevDependencies {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
