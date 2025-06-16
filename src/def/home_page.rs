use std::ops::{Deref, DerefMut};

use jsonc_parser::ParseResult;
use miette::MietteDiagnostic;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HomePage(pub String);

impl Deref for HomePage {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for HomePage {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl HomePage {
  pub fn validate(&self, _parse_result: &ParseResult) -> Option<MietteDiagnostic> {
    None
  }
}
