use std::ops::{Deref, DerefMut};

use jsonc_parser::{ParseResult, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Version(pub String);

impl Deref for Version {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Version {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Version {
  pub fn validate(&self, parse_result: &ParseResult) -> Option<MietteDiagnostic> {
    let regex = lazy_regex::regex_is_match!(
      r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$",
      &self
    );

    if regex {
      return None;
    }

    let range = parse_result
      .value
      .as_ref()
      .and_then(|v| v.as_object())
      .and_then(|obj| obj.get("version"))
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .and_then(|range| Some(range.start..range.end));

    let Some(range) = range else {
      return None;
    };

    let label = LabeledSpan::at(range, "here".to_string());
    let diagnostic = MietteDiagnostic::new(r"Package version does not match required pattern")
      .with_labels(vec![label])
      .with_help(r"Expected pattern: ^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$")
      .with_severity(Severity::Error)
      .with_code("E0001");

    return Some(diagnostic);
  }
}

#[cfg(test)]
mod tests {
  use crate::case::case;

  use super::*;

  #[test]
  fn should_pass_validate_version() {
    let json = r#"
    {
      "version": "1.0.0"
    }
    "#;

    let vs = vec![
      Version("1.0.0".to_string()),
      Version("1.0.0-alpha.1".to_string()),
      Version("1.0.0+build.1".to_string()),
      Version("1.0.0-alpha.1+build.1".to_string()),
    ];

    let parse_result = case(json).unwrap();

    for v in vs {
      let res = v.validate(&parse_result);
      assert!(res.is_none());
    }
  }

  #[test]
  fn should_fail_validate_version() {
    let json = r#"
    {
      "version": "1.0.0-alpha.1+build.1"
    }
    "#;

    let parse_result = case(json).unwrap();

    let vs = vec![Version("hello".to_string())];

    for v in vs {
      let res = v.validate(&parse_result);
      assert!(res.is_some());
    }
  }
}
