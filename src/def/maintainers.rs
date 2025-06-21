use crate::Person;
use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use miette::MietteDiagnostic;
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Deref, DerefMut)]
pub struct Maintainers(Vec<Person>);

impl Validator for Maintainers {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    todo!();
    vec![]
  }
}
