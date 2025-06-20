use derive_more::{Deref, DerefMut};
use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Deref, DerefMut)]
pub struct Name(pub String);

impl Validator for Name {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    let mut diagnostics = vec![];

    let regex = lazy_regex::regex_is_match!(
      r"^(?:(?:@(?:[a-z0-9-*~][a-z0-9-*._~]*)?/[a-z0-9-._~])|[a-z0-9-~])[a-z0-9-._~]*$",
      &self
    );

    if regex {
      return diagnostics;
    }

    // 更简洁的链式调用
    let range = prop
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .and_then(|range| Some(range.start..range.end));

    let Some(range) = range else {
      return diagnostics;
    };

    let label = LabeledSpan::at(range, "here".to_string());
    let diagnostic = MietteDiagnostic::new(r"Package name does not match required pattern")
            .with_labels(vec![label])
            .with_help(r"Expected pattern: ^(?:(?:@(?:[a-z0-9-*~][a-z0-9-*._~]*)?/[a-z0-9-._~])|[a-z0-9-~])[a-z0-9-._~]*$")
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
  fn should_pass_validate_name_with_regex() {
    t(&[r#"{"name": "test"}"#], |parser, parse_result| {
      let name = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|obj| obj.get("name"));
      let res = parser.name.unwrap().validate(name);
      assert!(res.len() == 0);
      res
    });
  }

  #[test]
  fn should_fail_validate_name_with_regex() {
    t(&[r#"{"name": "tesSSSt"}"#], |parser, parse_result| {
      let name = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|obj| obj.get("name"));
      let res = parser.name.unwrap().validate(name);
      assert!(res.len() == 1);
      res
    });
  }
}
