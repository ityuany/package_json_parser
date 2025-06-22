use std::ops::Range;

use derive_more::{Deref, DerefMut};
use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Clone, Deref, DerefMut)]
pub struct PackageManager(String);

impl PackageManager {
  fn get_package_manager_range(&self, prop: Option<&ObjectProp>) -> Option<Range<usize>> {
    prop
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }
}

impl Validator for PackageManager {
  fn validate(&self, prop: Option<&ObjectProp>) -> miette::Result<()> {
    let regex = lazy_regex::regex_is_match!(r#"(npm|pnpm|yarn|bun)@\d+\.\d+\.\d+(-.+)?"#, &self);

    if regex {
      return Ok(());
    }

    let range = self.get_package_manager_range(prop);

    let mut diagnostic = MietteDiagnostic::new("Invalid package manager".to_string())
      .with_help("Please provide a valid package manager")
      .with_severity(Severity::Error)
      .with_code("invalid_package_manager");

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
  fn should_pass_validate_package_manager() {
    let jsones = [
      r#"{"packageManager": "npm@1.0.0"}"#,
      r#"{"packageManager": "pnpm@1.0.0"}"#,
      r#"{"packageManager": "yarn@1.0.0"}"#,
      r#"{"packageManager": "bun@1.0.0"}"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_ok());
    }
  }

  #[test]
  fn should_fail_validate_package_manager() {
    let jsones = [r#"{"packageManager": "invalid"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_err());
    }
  }
}
