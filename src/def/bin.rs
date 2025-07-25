use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::ext::Validator;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Bin {
  String(String),
  Object(HashMap<String, String>),
}

impl Validator for Bin {
  fn validate(&self, _bin: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
