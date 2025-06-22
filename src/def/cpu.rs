use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, Serialize, Deserialize, Clone, Deref, DerefMut)]
pub struct Cpu(Vec<String>);

impl Validator for Cpu {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
