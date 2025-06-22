use std::ops::Range;

use derive_more::{Deref, DerefMut};
use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Deref, DerefMut)]
pub struct Name(String);

impl Name {
  fn get_name_range(&self, prop: Option<&ObjectProp>) -> Option<Range<usize>> {
    prop
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }
}

impl Validator for Name {
  fn validate(&self, prop: Option<&ObjectProp>) -> miette::Result<()> {
    let reg_name = lazy_regex::regex_is_match!(
      r"^(?:(?:@(?:[a-z0-9-*~][a-z0-9-*._~]*)?/[a-z0-9-._~])|[a-z0-9-~])[a-z0-9-._~]*$",
      &self
    );

    if reg_name {
      return Ok(());
    }

    let mut diagnostic = MietteDiagnostic::new(r"Package name does not match required pattern")
      .with_severity(Severity::Error)
      .with_help(r"Expected pattern: ^(?:(?:@(?:[a-z0-9-*~][a-z0-9-*._~]*)?/[a-z0-9-._~])|[a-z0-9-~])[a-z0-9-._~]*$")
      .with_code("invalid_package_name");

    // 更简洁的链式调用
    let range = self.get_name_range(prop);

    let Some(range) = range else {
      return Err(miette::miette!(diagnostic));
    };

    let label = LabeledSpan::at(range, "here".to_string());
    diagnostic = diagnostic.with_labels(vec![label]);

    return Err(miette::miette!(diagnostic));
  }
}

#[cfg(test)]
mod tests {

  use crate::PackageJsonParser;

  #[test]
  fn should_pass_validate_name_with_regex() {
    let jsones = [r#"{"name": "test"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_ok());
    }
  }

  #[test]
  fn should_fail_validate_name_with_regex() {
    let jsones = [r#"{"name": "tesSSSt"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_err());
    }
  }
}
