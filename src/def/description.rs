use std::ops::{Deref, DerefMut};

use jsonc_parser::ParseResult;
use miette::MietteDiagnostic;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Description(pub String);

impl Deref for Description {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Description {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Description {
  pub fn validate(&self, _parse_result: &ParseResult) -> Option<MietteDiagnostic> {
    None
  }
}
