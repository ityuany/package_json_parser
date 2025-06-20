use crate::ext::Validator;
use derive_more::{Deref, DerefMut};
use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Deref, DerefMut)]
pub struct License(pub String);

impl Validator for License {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    let mut diagnostics = vec![];

    let regex = lazy_regex::regex_is_match!(
      r"^(AGPL-3.0-only|Apache-2.0|BSD-2-Clause|BSD-3-Clause|BSL-1.0|CC0-1.0|CDDL-1.0|CDDL-1.1|EPL-1.0|EPL-2.0|GPL-2.0-only|GPL-3.0-only|ISC|LGPL-2.0-only|LGPL-2.1-only|LGPL-2.1-or-later|LGPL-3.0-only|LGPL-3.0-or-later|MIT|MPL-2.0|MSPL|UnLicense)$",
      &self
    );

    if !regex {
      let range = prop
        .and_then(|prop| prop.value.as_string_lit())
        .map(|value| value.range())
        .and_then(|range| Some(range.start..range.end));

      if let Some(range) = range {
        let label = LabeledSpan::at(range, "here".to_string());
        let diagnostic = MietteDiagnostic::new("Invalid license".to_string())
          .with_labels(vec![label])
          .with_severity(Severity::Error)
          .with_help("Please provide a valid license")
          .with_code("invalid_license");
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
  fn should_pass_validate_license() {
    let jsones = [
      r#"{"license": "MIT"}"#,
      r#"{"license": "Apache-2.0"}"#,
      r#"{"license": "BSD-2-Clause"}"#,
      r#"{"license": "BSD-3-Clause"}"#,
    ];
    t(&jsones, |parser, parse_result| {
      let license = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|obj| obj.get("license"));
      let res = parser.license.unwrap().validate(license);
      assert!(res.is_empty());
      res
    });
  }

  #[test]
  fn should_fail_when_license_is_invalid() {
    let jsones = [r#"{"license": "MIT1"}"#];
    t(&jsones, |parser, parse_result| {
      let license = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|obj| obj.get("license"));
      let res = parser.license.unwrap().validate(license);
      assert!(!res.is_empty());
      res
    });
  }
}
