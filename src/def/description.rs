use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use miette::MietteDiagnostic;
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Deref, DerefMut)]
pub struct Description(String);

impl Validator for Description {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    vec![]
  }
}
