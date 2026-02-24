use std::collections::HashMap;

use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Deserializer, Serialize};

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone, Deref, DerefMut)]
pub struct Engines(HashMap<String, String>);

impl<'de> Deserialize<'de> for Engines {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    HashMap::<String, String>::deserialize(deserializer).map(Self)
  }
}

impl Validator for Engines {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
