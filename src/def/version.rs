use std::ops::Range;

use derive_more::{Deref, DerefMut};
use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Deref, DerefMut)]
pub struct Version(String);

impl Version {
  fn get_version_range(&self, prop: Option<&ObjectProp>) -> Option<Range<usize>> {
    prop
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }
}

impl Validator for Version {
  fn validate(&self, prop: Option<&ObjectProp>) -> miette::Result<()> {
    let regex = lazy_regex::regex_is_match!(
      r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$",
      &self
    );

    if regex {
      return Ok(());
    }

    let range = self.get_version_range(prop);

    let mut diagnostic = MietteDiagnostic::new(r"Package version does not match required pattern")
      .with_help(r"Expected pattern: ^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$")
      .with_severity(Severity::Error)
      .with_code("E0001");

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
  fn should_pass_validate_version() {
    let jsones = [
      r#"{"version": "1.0.0"}"#,
      r#"{"version": "1.0.0-alpha.1"}"#,
      r#"{"version": "1.0.0+build.1"}"#,
      r#"{"version": "1.0.0-alpha.1+build.1"}"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_ok());
    }
  }

  #[test]
  fn should_fail_validate_version() {
    let jsones = [r#"{"version": "hello"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_err());
    }
  }
}
