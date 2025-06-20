use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use miette::MietteDiagnostic;
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, Serialize, Deserialize, Clone, Deref, DerefMut)]
pub struct Cpu(pub Vec<String>);

impl Cpu {
  pub fn validate(&self, _cpu: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    vec![]
  }
}

impl Validator for Cpu {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    vec![]
  }
}
