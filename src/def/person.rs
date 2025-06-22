use crate::ext::Validator;
use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};
use std::ops::Range;
use validator::{ValidateEmail, ValidateUrl};

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

impl Person {
  fn get_person_string_lit_range(&self, prop: Option<&ObjectProp>) -> Option<Range<usize>> {
    prop
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }

  fn get_person_object_name_range(&self, prop: Option<&ObjectProp>) -> Option<Range<usize>> {
    prop
      .and_then(|prop| prop.value.as_object())
      .and_then(|obj| obj.get("name"))
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }

  fn get_person_object_email_range(&self, prop: Option<&ObjectProp>) -> Option<Range<usize>> {
    prop
      .and_then(|prop| prop.value.as_object())
      .and_then(|obj| obj.get("email"))
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }

  fn get_person_object_url_range(&self, prop: Option<&ObjectProp>) -> Option<Range<usize>> {
    prop
      .and_then(|prop| prop.value.as_object())
      .and_then(|obj| obj.get("url"))
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }
}

impl Validator for Person {
  fn validate(&self, prop: Option<&ObjectProp>) -> miette::Result<()> {
    match self {
      Person::String(name) => {
        if !name.is_empty() {
          return Ok(());
        }

        let range = self.get_person_string_lit_range(prop);

        let mut diagnostic = MietteDiagnostic::new("Invalid name".to_string())
          .with_help("Please provide a valid name")
          .with_severity(Severity::Error)
          .with_code("invalid_name");

        if let Some(range) = range {
          let label = LabeledSpan::at(range, "Invalid name");
          diagnostic = diagnostic.with_labels(vec![label]);
        }

        return Err(miette::miette!(diagnostic));
      }
      Person::Object(person) => {
        if person.name.is_empty() {
          let mut diagnostic = MietteDiagnostic::new("Invalid name".to_string())
            .with_help("Please provide a valid name")
            .with_severity(Severity::Error)
            .with_code("invalid_name");

          let range = self.get_person_object_name_range(prop);

          if let Some(range) = range {
            let label = LabeledSpan::at(range, "Invalid name");
            diagnostic = diagnostic.with_labels(vec![label]);
          }

          return Err(miette::miette!(diagnostic));
        }

        if let Some(email) = person.email.as_ref() {
          if !email.validate_email() {
            let mut diagnostic = MietteDiagnostic::new("Invalid email".to_string())
              .with_help("Please provide a valid email")
              .with_severity(Severity::Error)
              .with_code("invalid_email");

            let range = self.get_person_object_email_range(prop);

            if let Some(range) = range {
              let label = LabeledSpan::at(range, "Invalid email");
              diagnostic = diagnostic.with_labels(vec![label]);
            }

            return Err(miette::miette!(diagnostic));
          }
        }

        if let Some(url) = person.url.as_ref() {
          if !url.validate_url() {
            let mut diagnostic = MietteDiagnostic::new("Invalid URL".to_string())
              .with_help("Please provide a valid URL")
              .with_severity(Severity::Error)
              .with_code("invalid_url");

            let range = self.get_person_object_url_range(prop);

            if let Some(range) = range {
              let label = LabeledSpan::at(range, "Invalid URL");
              diagnostic = diagnostic.with_labels(vec![label]);
            }

            return Err(miette::miette!(diagnostic));
          }
        }

        Ok(())
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
      let res = res.validate();
      assert!(res.is_err());
    }
  }
}
