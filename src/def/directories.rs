use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Directories {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bin: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lib: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub man: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub doc: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub example: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub test: Option<String>,
}

impl Validator for Directories {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
