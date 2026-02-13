use jsonc_parser::ast::ObjectProp;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
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

#[derive(Debug, Serialize, Clone)]
pub enum RepositoryOrString {
  Repository(Repository),
  String(String),
}

impl<'de> Deserialize<'de> for RepositoryOrString {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct RepositoryVisitor;

    impl<'de> Visitor<'de> for RepositoryVisitor {
      type Value = RepositoryOrString;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
          "a string or object `{ \"type\"?: string, \"url\"?: string, \"directory\"?: string }` for `repository`",
        )
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(RepositoryOrString::String(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(RepositoryOrString::String(value))
      }

      fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        let repository =
          Repository::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
        Ok(RepositoryOrString::Repository(repository))
      }
    }

    deserializer.deserialize_any(RepositoryVisitor)
  }
}

impl Validator for RepositoryOrString {
  fn validate(&self, repository: Option<&ObjectProp>) -> Vec<crate::validation::RuleViolation> {
    match self {
      RepositoryOrString::Repository(repos) => {
        let mut violations = Vec::new();
        if let Some(url) = repos.url.as_ref() {
          if !url.validate_url() {
            violations.push(validation_error(
              "Invalid url",
              Some("invalid_url"),
              "Please provide a valid url",
              value_range(repository, &["url"]),
              "url",
            ));
          }
        }
        violations
      }
      RepositoryOrString::String(string) => {
        if !string.validate_url() {
          return vec![validation_error(
            "Invalid url",
            Some("invalid_url"),
            "Please provide a valid url",
            value_range(repository, &[]),
            "",
          )];
        }
        vec![]
      }
    }
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
      let report = res.validate().unwrap();
      assert!(!report.has_errors());
      let repository = res.get_repository();
      assert!(repository.value.is_some());
      assert!(!repository.has_errors());
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
      let report = res.validate().unwrap();
      assert!(report.has_errors());
    }
  }
}
