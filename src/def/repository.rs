use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

use crate::ext::Validator;
use crate::validator::ValidatorUtil;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repository {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub r#type: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub directory: Option<String>,
}

impl Validator for Repository {
  fn validate(&self, repos: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    let mut diagnostics = vec![];

    if let Some(url) = self.url.as_ref() {
      if !ValidatorUtil::is_url(url) {
        let mut labels = vec![];
        if let Some(range) = repos
          .and_then(|prop| prop.value.as_object())
          .and_then(|obj| obj.get("url"))
          .and_then(|prop| prop.value.as_string_lit())
          .map(|value| value.range())
          .and_then(|range| Some(range.start..range.end))
        {
          labels.push(LabeledSpan::at(range, "Invalid url"));
        }
        let diagnostic = MietteDiagnostic::new("Invalid url".to_string())
          .with_labels(labels)
          .with_severity(Severity::Error)
          .with_help("Please provide a valid url");
        diagnostics.push(diagnostic);
      }
    }

    diagnostics
  }
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
#[serde(untagged)]
pub enum RepositoryOrString {
  Repository(Repository),
  String(String),
}

impl RepositoryOrString {
  pub fn validate(&self, repository: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    let mut diagnostics = vec![];
    match self {
      RepositoryOrString::Repository(repos) => {
        diagnostics.extend(repos.validate(repository));
      }
      RepositoryOrString::String(string) => {
        if !ValidatorUtil::is_url(string) {
          let mut labels = vec![];
          if let Some(range) = repository
            .and_then(|prop| prop.value.as_string_lit())
            .map(|value| value.range())
            .and_then(|range| Some(range.start..range.end))
          {
            labels.push(LabeledSpan::at(range, "Invalid url"));
          }
          let diagnostic = MietteDiagnostic::new("Invalid url".to_string())
            .with_labels(labels)
            .with_severity(Severity::Error)
            .with_help("Please provide a valid url");
          diagnostics.push(diagnostic);
        }
      }
    }
    diagnostics
  }
}

#[cfg(test)]
mod tests {
  use crate::case::t;

  #[test]
  fn should_pass_validate_repository() {
    let jsones = [
      r#"{"repository": {"type": "git", "url": "https://github.com/rust-lang/rust", "directory": "src"}}"#,
      r#"{"repository": "https://github.com/rust-lang/rust"}"#,
    ];

    t(&jsones, |parser, parse_result| {
      let repository = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|v| v.get("repository"));
      let res = parser.repository.unwrap().validate(repository);
      assert!(res.is_empty());
      res
    });
  }

  #[test]
  fn should_fail_validate_repository() {
    let jsones = [
      r#"{"repository": {"type": "git", "url": "invalid", "directory": "src"}}"#,
      r#"{"repository": "invalid"}"#,
    ];

    t(&jsones, |parser, parse_result| {
      let repository = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|v| v.get("repository"));
      let res = parser.repository.unwrap().validate(repository);
      assert!(!res.is_empty());
      res
    });
  }
}
