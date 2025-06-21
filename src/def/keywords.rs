use crate::ext::Validator;
use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use miette::MietteDiagnostic;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Deref, DerefMut)]
pub struct Keywords(Vec<String>);

impl Validator for Keywords {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    vec![]
  }
}
