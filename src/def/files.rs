use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Deserializer, Serialize};

use crate::ext::Validator;

#[derive(Debug, Serialize, Eq, PartialEq, Clone, Deref, DerefMut)]
pub struct Files(Vec<String>);

impl<'de> Deserialize<'de> for Files {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    Vec::<String>::deserialize(deserializer).map(Self)
  }
}

impl Validator for Files {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
