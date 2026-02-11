use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};
use validator::{ValidateEmail, ValidateUrl};

use crate::ext::{Validator, validation_error, value_range};

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
  fn validate(&self, props: Option<&ObjectProp>) -> miette::Result<()> {
    match self {
      Bugs::UrlOrEmail(value) => {
        if value.validate_url() || value.validate_email() {
          return Ok(());
        }

        return Err(validation_error(
          "Invalid URL or email",
          Some("invalid_url_or_email"),
          "Please provide a valid URL or email",
          value_range(props, &[]),
          "Invalid URL or email",
        ));
      }
      Bugs::BugsItem(bugs_item) => {
        if let Some(url) = bugs_item.url.as_ref() {
          if !url.validate_url() {
            return Err(validation_error(
              "Invalid URL",
              Some("invalid_url"),
              "Please provide a valid URL",
              value_range(props, &["url"]),
              "Invalid URL",
            ));
          }
        }

        if let Some(email) = bugs_item.email.as_ref() {
          if !email.validate_email() {
            return Err(validation_error(
              "Invalid Email",
              Some("invalid_email"),
              "Please provide a valid Email",
              value_range(props, &["email"]),
              "Invalid Email",
            ));
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
