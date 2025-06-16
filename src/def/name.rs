use std::ops::{Deref, DerefMut};

use jsonc_parser::{ParseResult, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Name(pub String);

impl Deref for Name {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Name {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Name {
  pub fn validate(&self, parse_result: &ParseResult) -> Option<MietteDiagnostic> {
    let regex = lazy_regex::regex_is_match!(
      r"^(?:(?:@(?:[a-z0-9-*~][a-z0-9-*._~]*)?/[a-z0-9-._~])|[a-z0-9-~])[a-z0-9-._~]*$",
      &self
    );

    if regex {
      return None;
    }

    // 更简洁的链式调用
    let range = parse_result
      .value
      .as_ref()
      .and_then(|v| v.as_object())
      .and_then(|obj| obj.get("name"))
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .and_then(|range| Some(range.start..range.end));

    let Some(range) = range else {
      return None;
    };

    let label = LabeledSpan::at(range, "here".to_string());
    let diagnostic = MietteDiagnostic::new(r"Package name does not match required pattern")
            .with_labels(vec![label])
            .with_help(r"Expected pattern: ^(?:(?:@(?:[a-z0-9-*~][a-z0-9-*._~]*)?/[a-z0-9-._~])|[a-z0-9-~])[a-z0-9-._~]*$")
            .with_severity(Severity::Error)
            .with_code("E0001");

    return Some(diagnostic);
  }
}

#[cfg(test)]
mod tests {

  use crate::case;

  use super::*;

  #[test]
  fn should_pass_validate_name_with_regex() {
    let json = r#"
    {
      "name": "tesSSSt"
    }
    "#;

    let parse_result = case::case(json).unwrap();

    let name = Name("tesSSSt".to_string());
    let res = name.validate(&parse_result);

    assert!(res.is_some());

    let name = Name("test".to_string());
    let res = name.validate(&parse_result);
    assert!(res.is_none());
  }
}
