use jsonc_parser::ast::ObjectProp;

pub trait Validator {
  fn validate(&self, prop: Option<&ObjectProp>) -> miette::Result<()>;
}
