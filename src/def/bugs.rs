use std::vec;

use jsonc_parser::{ParseResult, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BugsItem {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bugs {
  UrlOrEmail(String),
  BugsItem(BugsItem),
}

impl Bugs {
  pub fn validate(&self, parse_result: &ParseResult) -> Vec<MietteDiagnostic> {
    let mut diagnostics = vec![];

    match self {
      Bugs::UrlOrEmail(value) => {
        let is_url = lazy_regex::regex_is_match!(r"^https?://", value);
        let is_email =
          lazy_regex::regex_is_match!(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$", value);
        println!("!is_url && !is_email: {}", !is_url && !is_email);
        if !is_url && !is_email {
          let range = parse_result
            .value
            .as_ref()
            .and_then(|v| v.as_object())
            .and_then(|obj| obj.get("bugs"))
            .and_then(|prop| prop.value.as_string_lit())
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
        if let Some(url) = bugs_item.url.as_ref() {
          let is_url = lazy_regex::regex_is_match!(r"^https?://", url);

          if !is_url {
            let range = parse_result
              .value
              .as_ref()
              .and_then(|v| v.as_object())
              .and_then(|obj| obj.get("bugs"))
              .and_then(|prop| prop.value.as_object())
              .and_then(|obj| obj.get("url"))
              .and_then(|prop| prop.value.as_string_lit())
              .map(|value| value.range())
              .and_then(|range| Some(range.start..range.end));

            let labels = if let Some(range) = range {
              vec![LabeledSpan::at(range, "Invalid URL")]
            } else {
              vec![]
            };

            let diagnostic = MietteDiagnostic::new("Invalid URL".to_string())
              .with_labels(labels)
              .with_severity(Severity::Error)
              .with_help("Please provide a valid URL")
              .with_code("invalid_url");

            diagnostics.push(diagnostic);
          }
        }

        if let Some(email) = bugs_item.email.as_ref() {
          let is_email =
            lazy_regex::regex_is_match!(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$", email);

          if !is_email {
            let range = parse_result
              .value
              .as_ref()
              .and_then(|v| v.as_object())
              .and_then(|obj| obj.get("bugs"))
              .and_then(|prop| prop.value.as_object())
              .and_then(|obj| obj.get("email"))
              .and_then(|prop| prop.value.as_string_lit())
              .map(|value| value.range())
              .and_then(|range| Some(range.start..range.end));

            let labels = if let Some(range) = range {
              vec![LabeledSpan::at(range, "Invalid email")]
            } else {
              vec![]
            };

            let diagnostic = MietteDiagnostic::new("Invalid email".to_string())
              .with_labels(labels)
              .with_severity(Severity::Error)
              .with_help("Please provide a valid email")
              .with_code("invalid_email");

            diagnostics.push(diagnostic);
          }
        }

        diagnostics
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::case;

  use super::*;

  #[test]
  fn should_pass_validate_bugs_item() {
    let parse_result = case::case(
      r#"
    {
      "bugs": {
        "url": "https://example.com",
        "email": "test@example.com"
      }
    }"#,
    )
    .unwrap();

    let bugs = vec![
      Bugs::BugsItem(BugsItem {
        url: Some("https://example.com".to_string()),
        email: Some("test@example.com".to_string()),
      }),
      Bugs::UrlOrEmail("https://example.com".to_string()),
      Bugs::UrlOrEmail("test@example.com".to_string()),
    ];

    for bug in bugs {
      let res = bug.validate(&parse_result);
      assert!(res.is_empty());
    }
  }

  #[test]
  fn should_fail_validate_bugs_item() {
    let parse_result = case::case(
      r#"
    {
      "bugs": {
        "url": "invalid",
        "email": "test@example.com"
      }
    }"#,
    )
    .unwrap();

    let bugs = vec![
      Bugs::BugsItem(BugsItem {
        url: Some("invalid".to_string()),
        email: Some("test@example.com".to_string()),
      }),
      Bugs::UrlOrEmail("invalid".to_string()),
    ];

    for bug in bugs {
      let res = bug.validate(&parse_result);
      println!("{:#?}", res);
      assert!(!res.is_empty());
    }
  }
}
