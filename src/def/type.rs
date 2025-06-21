use derive_more::{Deref, DerefMut};
use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Deref, DerefMut)]
pub struct Type(String);

impl Validator for Type {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    let mut diagnostics = vec![];

    let regex = lazy_regex::regex_is_match!(r"^(commonjs|module)$", &self);

    if regex {
      return diagnostics;
    }

    let range = prop
      .and_then(|v| v.value.as_string_lit())
      .map(|v| v.range())
      .and_then(|range| Some(range.start..range.end));

    if let Some(range) = range {
      let mut labels = vec![];

      labels.push(LabeledSpan::at(range, "Invalid type"));

      diagnostics.push(
        MietteDiagnostic::new("Invalid type".to_string())
          .with_labels(labels)
          .with_severity(Severity::Error)
          .with_help("Please provide a valid type")
          .with_code("invalid_type"),
      );
    }

    diagnostics
  }
}

#[cfg(test)]
mod tests {
  use crate::{case::t, ext::Validator};

  #[test]
  fn should_pass_validate_type() {
    let jsones = [r#"{"type": "commonjs"}"#, r#"{"type": "module"}"#];

    t(&jsones, |parser, parse_result| {
      let r#type = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|obj| obj.get("type"));

      let res = parser.r#type.unwrap().validate(r#type);
      assert!(res.is_empty());
      res
    });
  }

  #[test]
  fn should_fail_validate_type() {
    let jsones = [r#"{"type": "invalid"}"#];

    t(&jsones, |parser, parse_result| {
      let r#type = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|obj| obj.get("type"));

      let res = parser.r#type.unwrap().validate(r#type);
      assert!(!res.is_empty());
      res
    });
  }
}
