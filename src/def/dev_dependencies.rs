use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Deserializer, Serialize};

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone, Deref, DerefMut, Default)]
pub struct DevDependencies(FxHashMap<String, String>);

impl<'de> Deserialize<'de> for DevDependencies {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    FxHashMap::<String, String>::deserialize(deserializer).map(Self)
  }
}

impl Validator for DevDependencies {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
