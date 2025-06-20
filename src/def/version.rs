use derive_more::{Deref, DerefMut};
use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Deref, DerefMut)]
pub struct Version(pub String);

impl Validator for Version {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    let mut diagnostics = vec![];

    let regex = lazy_regex::regex_is_match!(
      r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$",
      &self
    );

    if regex {
      return diagnostics;
    }

    let range = prop
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .and_then(|range| Some(range.start..range.end));

    let Some(range) = range else {
      return diagnostics;
    };

    let label = LabeledSpan::at(range, "here".to_string());
    let diagnostic = MietteDiagnostic::new(r"Package version does not match required pattern")
      .with_labels(vec![label])
      .with_help(r"Expected pattern: ^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$")
      .with_severity(Severity::Error)
      .with_code("E0001");

    diagnostics.push(diagnostic);
    diagnostics
  }
}

#[cfg(test)]
mod tests {
  use crate::{case::t, ext::Validator};

  #[test]
  fn should_pass_validate_version() {
    let jsones = [
      r#"{"version": "1.0.0"}"#,
      r#"{"version": "1.0.0-alpha.1"}"#,
      r#"{"version": "1.0.0+build.1"}"#,
      r#"{"version": "1.0.0-alpha.1+build.1"}"#,
    ];

    t(&jsones, |parser, parse_result| {
      let version = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|obj| obj.get("version"));
      let res = parser.version.unwrap().validate(version);
      assert!(res.len() == 0);
      res
    });
  }

  #[test]
  fn should_fail_validate_version() {
    let jsones = [r#"{"version": "hello"}"#];

    t(&jsones, |parser, parse_result| {
      let version = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|obj| obj.get("version"));
      let res = parser.version.unwrap().validate(version);
      assert!(res.len() == 1);
      res
    });
  }
}
