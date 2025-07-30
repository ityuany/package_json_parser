use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Readme {
  String(String),
  Object(ReadmeContent),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReadmeContent {
  pub r#type: String,
  pub value: String,
}

impl Validator for Readme {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}
