use crate::ext::Validator;
use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};
use std::{ops::Range, vec};
use validator::{ValidateEmail, ValidateUrl};

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

impl Bugs {
  fn get_bugs_string_lit_range(props: Option<&ObjectProp>) -> Option<Range<usize>> {
    props
      .and_then(|v| v.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }

  fn get_bugs_item_url_range(props: Option<&ObjectProp>) -> Option<Range<usize>> {
    props
      .and_then(|v| v.value.as_object())
      .and_then(|obj| obj.get("url"))
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }

  fn get_bugs_item_email_range(props: Option<&ObjectProp>) -> Option<Range<usize>> {
    props
      .and_then(|v| v.value.as_object())
      .and_then(|obj| obj.get("email"))
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }
}

impl Validator for Bugs {
  fn validate(&self, props: Option<&ObjectProp>) -> miette::Result<()> {
    match self {
      Bugs::UrlOrEmail(value) => {
        if value.validate_url() || value.validate_email() {
          return Ok(());
        }

        let mut diagnostic = MietteDiagnostic::new("Invalid URL or email".to_string())
          .with_severity(Severity::Error)
          .with_help("Please provide a valid URL or email")
          .with_code("invalid_url_or_email");

        let primary_span = Self::get_bugs_string_lit_range(props)
          .map(|r| LabeledSpan::at(r, "Invalid URL or email"));

        if let Some(primary_span) = primary_span {
          diagnostic = diagnostic.with_labels(vec![primary_span]);
        }

        return Err(miette::miette!(diagnostic));
      }
      Bugs::BugsItem(bugs_item) => {
        if let Some(url) = bugs_item.url.as_ref() {
          if !url.validate_url() {
            let range =
              Self::get_bugs_item_url_range(props).map(|r| LabeledSpan::at(r, "Invalid URL"));

            let mut diagnostic = MietteDiagnostic::new("Invalid URL".to_string())
              .with_severity(Severity::Error)
              .with_help("Please provide a valid URL")
              .with_code("invalid_url");

            if let Some(primary_span) = range {
              diagnostic = diagnostic.with_labels(vec![primary_span]);
            }

            return Err(miette::miette!(diagnostic));
          }
        }

        if let Some(email) = bugs_item.email.as_ref() {
          if !email.validate_email() {
            let range =
              Self::get_bugs_item_email_range(props).map(|r| LabeledSpan::at(r, "Invalid Email"));

            let mut diagnostic = MietteDiagnostic::new("Invalid Email".to_string())
              .with_severity(Severity::Error)
              .with_help("Please provide a valid Email")
              .with_code("invalid_email");

            if let Some(primary_span) = range {
              diagnostic = diagnostic.with_labels(vec![primary_span]);
            }

            return Err(miette::miette!(diagnostic));
          }
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
    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_ok());
    }
  }

  #[test]
  fn should_fail_validate_bugs_item() {
    let jsones = [
      r#"{"bugs": {"url": "invalid", "email": "test@example.com"}}"#,
      r#"{"bugs": {"url": "https://example.com", "email": "invalid"}}"#,
      r#"{"bugs": {"url": "invalid", "email": "invalid"}}"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_err());
    }
  }
}
