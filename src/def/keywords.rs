use std::ops::{Deref, DerefMut};

use jsonc_parser::ParseResult;
use miette::MietteDiagnostic;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Keywords(pub Vec<String>);

impl Deref for Keywords {
  type Target = Vec<String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Keywords {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Keywords {
  pub fn validate(&self, _parse_result: &ParseResult) -> Option<MietteDiagnostic> {
    None
  }
}
