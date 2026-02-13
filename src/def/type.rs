use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Deref, DerefMut)]
pub struct Type(String);

impl Validator for Type {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<crate::validation::RuleViolation> {
    let regex = lazy_regex::regex_is_match!(r"^(commonjs|module)$", &self);

    if regex {
      return vec![];
    }

    vec![validation_error(
      "Invalid type",
      Some("invalid_type"),
      "Please provide a valid type",
      value_range(prop, &[]),
      "",
    )]
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_pass_validate_type() {
    let jsones = [r#"{"type": "commonjs"}"#, r#"{"type": "module"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let report = res.validate().unwrap();
      assert!(!report.has_errors());
      let r#type = res.get_type();
      assert!(r#type.value.is_some());
      assert!(!r#type.has_errors());
    }
  }

  #[test]
  fn should_fail_validate_type() {
    let jsones = [r#"{"type": "invalid"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let report = res.validate().unwrap();
      assert!(report.has_errors());
    }
  }
}
