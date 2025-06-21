use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use miette::MietteDiagnostic;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, Serialize, Deserialize, Clone, Deref, DerefMut)]
pub struct Scripts(FxHashMap<String, String>);

impl Validator for Scripts {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    vec![]
  }
}
