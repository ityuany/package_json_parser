use jsonc_parser::ast::ObjectProp;
use miette::MietteDiagnostic;

pub trait Validator {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<MietteDiagnostic>;
}
