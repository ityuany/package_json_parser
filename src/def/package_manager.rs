use derive_more::{Deref, DerefMut};
use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Clone, Deref, DerefMut)]
pub struct PackageManager(pub String);

impl Validator for PackageManager {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    let mut diagnostics = vec![];

    let regex = lazy_regex::regex_is_match!(r#"(npm|pnpm|yarn|bun)@\d+\.\d+\.\d+(-.+)?"#, &self);

    if !regex {
      let range = prop
        .and_then(|prop| prop.value.as_string_lit())
        .map(|value| value.range())
        .and_then(|range| Some(range.start..range.end));

      if let Some(range) = range {
        let mut labels = vec![];

        labels.push(LabeledSpan::at(range, "Invalid package manager"));

        let diagnostic = MietteDiagnostic::new("Invalid package manager".to_string())
          .with_severity(Severity::Error)
          .with_help("Please provide a valid package manager")
          .with_labels(labels);

        diagnostics.push(diagnostic);
      }
    }
    diagnostics
  }
}

#[cfg(test)]
mod tests {
  use crate::{case::t, ext::Validator};

  #[test]
  fn should_pass_validate_package_manager() {
    let jsones = [
      r#"{"packageManager": "npm@1.0.0"}"#,
      r#"{"packageManager": "pnpm@1.0.0"}"#,
      r#"{"packageManager": "yarn@1.0.0"}"#,
      r#"{"packageManager": "bun@1.0.0"}"#,
    ];

    t(&jsones, |parser, parse_result| {
      let package_manager = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|obj| obj.get("packageManager"));

      let res = parser.package_manager.unwrap().validate(package_manager);
      assert!(res.is_empty());
      res
    });
  }

  #[test]
  fn should_fail_validate_package_manager() {
    let jsones = [r#"{"packageManager": "invalid"}"#];

    t(&jsones, |parser, parse_result| {
      let package_manager = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|obj| obj.get("packageManager"));

      let res = parser.package_manager.unwrap().validate(package_manager);
      assert!(!res.is_empty());
      res
    });
  }
}
