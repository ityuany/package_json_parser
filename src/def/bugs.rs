use std::vec;

use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BugsItem {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Bugs {
  UrlOrEmail(String),
  BugsItem(BugsItem),
}

impl Validator for Bugs {
  fn validate(&self, props: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    let mut diagnostics = vec![];

    match self {
      Bugs::UrlOrEmail(value) => {
        let is_url = lazy_regex::regex_is_match!(r"^https?://", value);
        let is_email =
          lazy_regex::regex_is_match!(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$", value);

        if !is_url && !is_email {
          let bugs = props.and_then(|v| v.value.as_string_lit());

          let range = bugs
            .map(|value| value.range())
            .and_then(|range| Some(range.start..range.end));

          let labels = if let Some(range) = range {
            vec![LabeledSpan::at(range, "Invalid URL or email")]
          } else {
            vec![]
          };

          let diagnostic = MietteDiagnostic::new("Invalid URL or email".to_string())
            .with_labels(labels)
            .with_severity(Severity::Error)
            .with_help("Please provide a valid URL or email")
            .with_code("invalid_url_or_email");

          diagnostics.push(diagnostic);
        }
        diagnostics
      }
      Bugs::BugsItem(bugs_item) => {
        let mut labels = vec![];

        if let Some(url) = bugs_item.url.as_ref() {
          println!("url: {}", url);

          let is_url = lazy_regex::regex_is_match!(r"^https?://", url);

          if !is_url {
            let range = props
              .and_then(|prop| prop.value.as_object())
              .and_then(|obj| obj.get("url"))
              .and_then(|prop| prop.value.as_string_lit())
              .map(|value| value.range())
              .and_then(|range| Some(range.start..range.end));

            if let Some(range) = range {
              labels.push(LabeledSpan::at(range, "Invalid URL"));
            }

            // let diagnostic = MietteDiagnostic::new("Invalid URL".to_string())
            //   .with_labels(labels)
            //   .with_severity(Severity::Error)
            //   .with_help("Please provide a valid URL")
            //   .with_code("invalid_url");

            // diagnostics.push(diagnostic);
          }
        }

        if let Some(email) = bugs_item.email.as_ref() {
          let is_email =
            lazy_regex::regex_is_match!(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$", email);

          if !is_email {
            let range = props
              .and_then(|prop| prop.value.as_object())
              .and_then(|obj| obj.get("email"))
              .and_then(|prop| prop.value.as_string_lit())
              .map(|value| value.range())
              .and_then(|range| Some(range.start..range.end));

            if let Some(range) = range {
              labels.push(LabeledSpan::at(range, "Invalid email"));
            }

            // let diagnostic = MietteDiagnostic::new("Invalid email".to_string())
            //   .with_labels(labels)
            //   .with_severity(Severity::Error)
            //   .with_help("Please provide a valid email")
            //   .with_code("invalid_email");

            // diagnostics.push(diagnostic);
          }
        }

        if !labels.is_empty() {
          let diagnostic = MietteDiagnostic::new("Invalid URL or email".to_string())
            .with_labels(labels)
            .with_severity(Severity::Error)
            .with_help("Please provide a valid URL or email")
            .with_code("invalid_url_or_email");
          diagnostics.push(diagnostic);
        }

        diagnostics
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{case::t, ext::Validator};

  #[test]
  fn should_pass_validate_bugs_item() {
    let jsones = [
      r#"
      {
        "bugs": {
          "url": "https://example.com",
          "email": "test@example.com"
        }
      }"#,
      r#"
      {
        "bugs": "https://example.com"
      }"#,
      r#"
      {
        "bugs": "test@example.com"
      }"#,
    ];

    t(&jsones, |parser, parse_result| {
      let bugs = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|obj| obj.get("bugs"));

      let res = parser.bugs.unwrap().validate(bugs);
      assert!(res.is_empty());
      res
    });
  }

  #[test]
  fn should_fail_validate_bugs_item() {
    let jsones = [
      r#"{"bugs": {"url": "invalid", "email": "test@example.com"}}"#,
      r#"{"bugs": {"url": "https://example.com", "email": "invalid"}}"#,
      r#"{"bugs": {"url": "invalid", "email": "invalid"}}"#,
    ];

    t(&jsones, |parser, parse_result| {
      let bugs = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|obj| obj.get("bugs"));
      let res = parser.bugs.unwrap().validate(bugs);
      assert!(!res.is_empty());
      res
    });
  }
}
