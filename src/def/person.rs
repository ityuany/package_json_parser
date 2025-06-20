use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

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
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    let mut diagnostics = vec![];

    match self {
      Person::String(name) => {
        if name.is_empty() {
          let mut labels = vec![];

          let range = prop
            .and_then(|prop| prop.value.as_string_lit())
            .map(|value| value.range())
            .and_then(|range| Some(range.start..range.end));

          if let Some(range) = range {
            labels.push(LabeledSpan::at(range, "Invalid name"));
          }

          if !labels.is_empty() {
            let diagnostic = MietteDiagnostic::new("Invalid name".to_string())
              .with_labels(labels)
              .with_severity(Severity::Error)
              .with_help("Please provide a valid name")
              .with_code("invalid_name");
            diagnostics.push(diagnostic);
          }
        }
        return diagnostics;
      }
      Person::Object(person) => {
        let author = prop.and_then(|v| v.value.as_object());

        let mut labels = vec![];

        if person.name.is_empty() {
          let range = author
            .and_then(|v| v.get("name"))
            .and_then(|v| v.value.as_string_lit())
            .map(|v| v.range())
            .and_then(|range| Some(range.start..range.end));

          if let Some(range) = range {
            labels.push(LabeledSpan::at(range, "Invalid name"));
          }
        }

        if let Some(email) = person.email.as_ref() {
          if !lazy_regex::regex_is_match!(
            r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
            email
          ) {
            let range = author
              .and_then(|obj| obj.get("email"))
              .and_then(|prop| prop.value.as_string_lit())
              .map(|value| value.range())
              .and_then(|range| Some(range.start..range.end));

            if let Some(range) = range {
              labels.push(LabeledSpan::at(range, "Invalid email"));
            }
          }
        }

        if let Some(url) = person.url.as_ref() {
          if !lazy_regex::regex_is_match!(r"^https?://", url) {
            let range = author
              .and_then(|obj| obj.get("url"))
              .and_then(|prop| prop.value.as_string_lit())
              .map(|value| value.range())
              .and_then(|range| Some(range.start..range.end));

            if let Some(range) = range {
              labels.push(LabeledSpan::at(range, "Invalid URL"));
            }
          }
        }

        if !labels.is_empty() {
          let diagnostic = MietteDiagnostic::new("Invalid email or URL".to_string())
            .with_labels(labels)
            .with_severity(Severity::Error)
            .with_help("Please provide a valid email or URL")
            .with_code("invalid_email_or_url");
          diagnostics.push(diagnostic);
        }
      }
    }

    diagnostics
  }
}

#[cfg(test)]
mod tests {
  use crate::{case::t, ext::Validator};

  #[test]
  fn should_pass_validate_person() {
    let jsones = [
      r#"{ "author": { "name": "test" } }"#,
      r#"{ "author": { "name": "test", "email": "test@example.com" } }"#,
      r#"{ "author": { "name": "test", "url": "https://example.com" } }"#,
      r#"{ "author": { "name": "test", "email": "test@example.com", "url": "https://example.com" } }"#,
    ];

    t(&jsones, |parser, parse_result| {
      let author = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|obj| obj.get("author"));

      let res = parser.author.unwrap().validate(author);
      assert!(res.is_empty());
      res
    });
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

    t(&jsones, |parser, parse_result| {
      let author = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|obj| obj.get("author"));

      let res = parser.author.unwrap().validate(author);
      assert!(!res.is_empty());
      res
    });
  }
}
