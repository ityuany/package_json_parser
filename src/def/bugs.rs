use jsonc_parser::ast::ObjectProp;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use validator::{ValidateEmail, ValidateUrl};

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BugsItem {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum Bugs {
  UrlOrEmail(String),
  BugsItem(BugsItem),
}

impl<'de> Deserialize<'de> for Bugs {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct BugsVisitor;

    impl<'de> Visitor<'de> for BugsVisitor {
      type Value = Bugs;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
          "a string (url/email) or object `{ \"url\"?: string, \"email\"?: string }` for `bugs`",
        )
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Bugs::UrlOrEmail(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Bugs::UrlOrEmail(value))
      }

      fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        let item = BugsItem::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
        Ok(Bugs::BugsItem(item))
      }
    }

    deserializer.deserialize_any(BugsVisitor)
  }
}

impl Validator for Bugs {
  fn validate(&self, props: Option<&ObjectProp>) -> Vec<crate::validation::RuleViolation> {
    match self {
      Bugs::UrlOrEmail(value) => {
        if value.validate_url() || value.validate_email() {
          return vec![];
        }

        return vec![validation_error(
          "Invalid URL or email",
          Some("invalid_url_or_email"),
          "Please provide a valid URL or email",
          value_range(props, &[]),
          "",
        )];
      }
      Bugs::BugsItem(bugs_item) => {
        let mut violations = Vec::new();

        if let Some(url) = bugs_item.url.as_ref() {
          if !url.validate_url() {
            violations.push(validation_error(
              "Invalid URL",
              Some("invalid_url"),
              "Please provide a valid URL",
              value_range(props, &["url"]),
              "url",
            ));
          }
        }

        if let Some(email) = bugs_item.email.as_ref() {
          if !email.validate_email() {
            violations.push(validation_error(
              "Invalid Email",
              Some("invalid_email"),
              "Please provide a valid Email",
              value_range(props, &["email"]),
              "email",
            ));
          }
        }

        return violations;
      }
    }
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
      let report = res.validate().unwrap();
      assert!(!report.has_errors());
      let bugs = res.get_bugs();
      assert!(bugs.value.is_some());
      assert!(!bugs.has_errors());
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
      let report = res.validate().unwrap();
      assert!(report.has_errors());
    }
  }
}
