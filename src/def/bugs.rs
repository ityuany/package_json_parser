use jsonc_parser::ast::ObjectProp;
use serde::de::{self, IgnoredAny, MapAccess, Visitor, value::MapAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use validator::{ValidateEmail, ValidateUrl};

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct BugsItem {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
}

impl<'de> Deserialize<'de> for BugsItem {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    const FIELDS: &[&str] = &["url", "email"];

    enum Field {
      Url,
      Email,
      Ignore,
    }

    impl<'de> Deserialize<'de> for Field {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: Deserializer<'de>,
      {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
          type Value = Field;

          fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("`url` or `email`")
          }

          fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
          where
            E: de::Error,
          {
            Ok(match value {
              "url" => Field::Url,
              "email" => Field::Email,
              _ => Field::Ignore,
            })
          }
        }

        deserializer.deserialize_identifier(FieldVisitor)
      }
    }

    struct BugsItemVisitor;

    impl<'de> Visitor<'de> for BugsItemVisitor {
      type Value = BugsItem;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object with optional `url` and `email`")
      }

      fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        let mut url = None;
        let mut email = None;
        let mut seen_url = false;
        let mut seen_email = false;

        while let Some(key) = map.next_key::<Field>()? {
          match key {
            Field::Url => {
              if seen_url {
                return Err(de::Error::duplicate_field("url"));
              }
              url = map.next_value()?;
              seen_url = true;
            }
            Field::Email => {
              if seen_email {
                return Err(de::Error::duplicate_field("email"));
              }
              email = map.next_value()?;
              seen_email = true;
            }
            Field::Ignore => {
              let _: IgnoredAny = map.next_value()?;
            }
          }
        }

        Ok(BugsItem { url, email })
      }
    }

    deserializer.deserialize_struct("BugsItem", FIELDS, BugsItemVisitor)
  }
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

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string or an object for bugs")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Bugs::UrlOrEmail(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Bugs::UrlOrEmail(value))
      }

      fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        let item = BugsItem::deserialize(MapAccessDeserializer::new(map))?;
        Ok(Bugs::BugsItem(item))
      }
    }

    deserializer.deserialize_any(BugsVisitor)
  }
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

  #[test]
  fn should_deserialize_bugs_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"bugs":{ "url": "https://example.com" }}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_bugs_when_field_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(
      r#"
      { "bugs":
        { 
          "url": true, 
          "email": "a@b.com" 
        } 
      }
    "#,
    );
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert!(parsed.validate().is_err());
  }

  #[test]
  fn should_fail_deserialize_bugs_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
