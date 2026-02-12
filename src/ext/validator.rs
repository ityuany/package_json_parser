use jsonc_parser::ast::ObjectProp;

use crate::validation::RuleViolation;

pub trait Validator {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<RuleViolation>;
}
