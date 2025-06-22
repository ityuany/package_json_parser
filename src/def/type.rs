use std::ops::Range;

use derive_more::{Deref, DerefMut};
use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Deref, DerefMut)]
pub struct Type(String);

impl Type {
  fn get_type_range(&self, prop: Option<&ObjectProp>) -> Option<Range<usize>> {
    prop
      .and_then(|v| v.value.as_string_lit())
      .map(|v| v.range())
      .and_then(|range| Some(range.start..range.end))
  }
}

impl Validator for Type {
  fn validate(&self, prop: Option<&ObjectProp>) -> miette::Result<()> {
    let regex = lazy_regex::regex_is_match!(r"^(commonjs|module)$", &self);

    if regex {
      return Ok(());
    }

    let range = self.get_type_range(prop);

    let mut diagnostic = MietteDiagnostic::new("Invalid type".to_string())
      .with_help("Please provide a valid type")
      .with_severity(Severity::Error)
      .with_code("invalid_type");

    if let Some(range) = range {
      let label = LabeledSpan::at(range, "here".to_string());
      diagnostic = diagnostic.with_labels(vec![label]);
    }

    return Err(miette::miette!(diagnostic));
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
      let res = res.validate();
      assert!(res.is_ok());
    }
  }

  #[test]
  fn should_fail_validate_type() {
    let jsones = [r#"{"type": "invalid"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_err());
    }
  }
}
