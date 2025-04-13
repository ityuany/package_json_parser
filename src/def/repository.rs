use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Repository {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub r#type: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub directory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(untagged)]
pub enum RepositoryOrString {
  Repository(Repository),
  String(String),
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_pass_when_repository_is_object() {
    let raw = r#"{"type": "git", "url": "https://github.com/rust-lang/rust", "directory": "src"}"#;
    let repository: RepositoryOrString = serde_json::from_str(raw).unwrap();

    assert!(repository.validate().is_ok());
  }

  #[test]
  fn should_pass_when_repository_is_string() {
    let raw = r#""https://github.com/rust-lang/rust""#;
    let repository: RepositoryOrString = serde_json::from_str(raw).unwrap();

    assert!(repository.validate().is_ok());
  }

  #[test]
  fn should_pass_validate_repository() {
    let repository = Repository {
      r#type: Some("git".to_string()),
      url: Some("https://github.com/rust-lang/rust".to_string()),
      directory: Some("src".to_string()),
    };
    let res = repository.validate();

    assert!(res.is_ok());
  }

  #[test]
  fn should_pass_validate_repository_or_string() {
    let repository = RepositoryOrString::Repository(Repository {
      r#type: Some("git".to_string()),
      url: Some("https://github.com/rust-lang/rust".to_string()),
      directory: Some("src".to_string()),
    });
    let res = repository.validate();

    assert!(res.is_ok());
  }

  #[test]
  fn should_pass_deserialize_repository_or_string() {
    let raw = r#"{"type": "git", "url": "https://github.com/rust-lang/rust", "directory": "src"}"#;
    let repository: RepositoryOrString = serde_json::from_str(raw).unwrap();

    assert!(repository.validate().is_ok());
  }

  #[test]
  fn should_pass_deserialize_repository_or_string_string() {
    let raw = r#""https://github.com/rust-lang/rust""#;
    let repository: RepositoryOrString = serde_json::from_str(raw).unwrap();

    assert!(repository.validate().is_ok());
  }
}
