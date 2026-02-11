use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};
use validator::ValidateUrl;

use crate::ext::{Validator, validation_error, value_range};

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
  fn validate(&self, repos: Option<&ObjectProp>) -> miette::Result<()> {
    if let Some(url) = self.url.as_ref() {
      if !url.validate_url() {
        return Err(validation_error(
          "Invalid url",
          Some("invalid_url"),
          "Please provide a valid url",
          value_range(repos, &["url"]),
          "Invalid url",
        ));
      }
    }
    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RepositoryOrString {
  Repository(Repository),
  String(String),
}

impl RepositoryOrString {
  pub fn validate(&self, repository: Option<&ObjectProp>) -> miette::Result<()> {
    match self {
      RepositoryOrString::Repository(repos) => {
        return repos.validate(repository);
      }
      RepositoryOrString::String(string) => {
        if !string.validate_url() {
          return Err(validation_error(
            "Invalid url",
            Some("invalid_url"),
            "Please provide a valid url",
            value_range(repository, &[]),
            "Invalid url",
          ));
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
