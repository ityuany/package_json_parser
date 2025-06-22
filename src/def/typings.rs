use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Deref, DerefMut)]
pub struct Typings(String);

impl Validator for Typings {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
