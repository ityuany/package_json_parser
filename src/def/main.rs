// use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

// #[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Deref, DerefMut)]
// pub struct Main(String);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Main {
  Bool(bool),
  Str(String),
}

impl Validator for Main {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
