use jsonc_parser::ast::ObjectProp;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use validator::{ValidateEmail, ValidateUrl};

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
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

impl<'de> Deserialize<'de> for Person {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct PersonVisitor;

    impl<'de> Visitor<'de> for PersonVisitor {
      type Value = Person;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
          "a string or object `{ \"name\": string, \"email\"?: string, \"url\"?: string }`",
        )
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Person::String(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(Person::String(value))
      }

      fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        let person = PersonObject::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
        Ok(Person::Object(person))
      }
    }

    deserializer.deserialize_any(PersonVisitor)
  }
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
      let report = res.validate().unwrap();
      assert!(!report.has_errors());
      let author = res.get_author();
      assert!(author.value.is_some());
      assert!(!author.has_errors());
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
      let report = res.validate().unwrap();
      assert!(report.has_errors());
    }
  }
}
