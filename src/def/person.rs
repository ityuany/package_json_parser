use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};
use validator::{ValidateEmail, ValidateUrl};

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum Person {
  String(String),
  Object(PersonObject),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct PersonObject {
  pub name: String,
  pub email: Option<String>,
  pub url: Option<String>,
}

impl Validator for Person {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<crate::validation::RuleViolation> {
    match self {
      Person::String(name) => {
        if name.is_empty() {
          return vec![validation_error(
            "Invalid name",
            Some("invalid_name"),
            "Please provide a valid name",
            value_range(prop, &[]),
            "",
          )];
        }
        vec![]
      }
      Person::Object(person) => {
        let mut violations = Vec::new();

        if person.name.is_empty() {
          violations.push(validation_error(
            "Invalid name",
            Some("invalid_name"),
            "Please provide a valid name",
            value_range(prop, &["name"]),
            "name",
          ));
        }

        if let Some(email) = person.email.as_ref() {
          if !email.validate_email() {
            violations.push(validation_error(
              "Invalid email",
              Some("invalid_email"),
              "Please provide a valid email",
              value_range(prop, &["email"]),
              "email",
            ));
          }
        }

        if let Some(url) = person.url.as_ref() {
          if !url.validate_url() {
            violations.push(validation_error(
              "Invalid URL",
              Some("invalid_url"),
              "Please provide a valid URL",
              value_range(prop, &["url"]),
              "url",
            ));
          }
        }

        violations
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_pass_validate_person() {
    let jsones = [
      r#"{ "author": { "name": "test" } }"#,
      r#"{ "author": { "name": "test", "email": "test@example.com" } }"#,
      r#"{ "author": { "name": "test", "url": "https://example.com" } }"#,
      r#"{ "author": { "name": "test", "email": "test@example.com", "url": "https://example.com" } }"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_ok());
    }
  }

  #[test]
  fn should_fail_validate_person() {
    let jsones = [
      r#"{ "author": { "name": "test", "email": "invalid" } }"#,
      r#"{ "author": { "name": "test", "url": "invalid" } }"#,
      r#"{ "author": { "name": "test", "url": "invalid", "email": "invalid" } }"#,
      r#"{ "author": { "name": "", "url": "invalid", "email": "invalid" } }"#,
      r#"{ "author": "" }"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let report = res.validate_with(crate::ValidationOptions::error()).unwrap();
      assert!(report.has_errors());
    }
  }
}
