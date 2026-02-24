use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Deserializer, Serialize};

use crate::ext::Validator;

#[derive(Debug, PartialEq, Serialize, Clone, Deref, DerefMut)]
pub struct Description(String);

impl<'de> Deserialize<'de> for Description {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    String::deserialize(deserializer).map(Self)
  }
}

impl Validator for Description {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
