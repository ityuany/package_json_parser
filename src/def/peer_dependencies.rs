use crate::ext::Validator;
use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use rustc_hash::FxHashMap;
use serde::de::{MapAccess, Visitor, value::MapAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Clone, Deref, DerefMut)]
pub struct PeerDependencies(FxHashMap<String, String>);

impl<'de> Deserialize<'de> for PeerDependencies {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct PeerDependenciesVisitor;

    impl<'de> Visitor<'de> for PeerDependenciesVisitor {
      type Value = PeerDependencies;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object map for peerDependencies")
      }

      fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        let value = FxHashMap::<String, String>::deserialize(MapAccessDeserializer::new(map))?;
        Ok(PeerDependencies(value))
      }
    }

    deserializer.deserialize_any(PeerDependenciesVisitor)
  }
}

impl Validator for PeerDependencies {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
