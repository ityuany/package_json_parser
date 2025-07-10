use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Module {
//   pub r#type: String,
//   pub value: String,
// }

#[derive(Debug, Serialize, Deserialize, Clone, Deref, DerefMut)]
pub struct Module(String);

impl Validator for Module {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
