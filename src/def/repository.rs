use std::ops::Range;

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

impl Repository {
  fn get_repository_url_range(&self, repos: Option<&ObjectProp>) -> Option<Range<usize>> {
    repos
      .and_then(|prop| prop.value.as_object())
      .and_then(|obj| obj.get("url"))
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }
}

impl Validator for Repository {
  fn validate(&self, repos: Option<&ObjectProp>) -> miette::Result<()> {
    if let Some(url) = self.url.as_ref() {
      if !ValidatorUtil::is_url(url) {
        let mut diagnostic = MietteDiagnostic::new("Invalid url".to_string())
          .with_severity(Severity::Error)
          .with_help("Please provide a valid url")
          .with_code("invalid_url");

        if let Some(range) = self.get_repository_url_range(repos) {
          let label = LabeledSpan::at(range, "Invalid url");
          diagnostic = diagnostic.with_labels(vec![label]);
        }

        return Err(miette::miette!(diagnostic));
      }
    }
    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
#[serde(untagged)]
pub enum RepositoryOrString {
  Repository(Repository),
  String(String),
}

impl RepositoryOrString {
  fn get_repository_string_lit_range(
    &self,
    repository: Option<&ObjectProp>,
  ) -> Option<Range<usize>> {
    repository
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }
}
impl RepositoryOrString {
  pub fn validate(&self, repository: Option<&ObjectProp>) -> miette::Result<()> {
    match self {
      RepositoryOrString::Repository(repos) => {
        return repos.validate(repository);
      }
      RepositoryOrString::String(string) => {
        if !ValidatorUtil::is_url(string) {
          let mut diagnostic = MietteDiagnostic::new("Invalid url".to_string())
            .with_severity(Severity::Error)
            .with_help("Please provide a valid url")
            .with_code("invalid_url");

          if let Some(range) = self.get_repository_string_lit_range(repository) {
            let label = LabeledSpan::at(range, "Invalid url");
            diagnostic = diagnostic.with_labels(vec![label]);
          }

          return Err(miette::miette!(diagnostic));
        }
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_pass_validate_repository() {
    let jsones = [
      r#"{"repository": {"type": "git", "url": "https://github.com/rust-lang/rust", "directory": "src"}}"#,
      r#"{"repository": "https://github.com/rust-lang/rust"}"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_ok());
    }
  }

  #[test]
  fn should_fail_validate_repository() {
    let jsones = [
      r#"{"repository": {"type": "git", "url": "invalid", "directory": "src"}}"#,
      r#"{"repository": "invalid"}"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_err());
    }
  }
}
