use crate::ext::Validator;
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Keywords {
  String(String),
  Array(Vec<String>),
}

impl Validator for Keywords {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
