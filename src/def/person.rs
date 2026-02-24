use jsonc_parser::ast::ObjectProp;
use serde::de::{self, IgnoredAny, MapAccess, Visitor, value::MapAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use validator::{ValidateEmail, ValidateUrl};

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub enum Person {
  String(String),
  Object(PersonObject),
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub struct PersonObject {
  pub name: String,
  pub email: Option<String>,
  pub url: Option<String>,
}

impl<'de> Deserialize<'de> for PersonObject {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    const FIELDS: &[&str] = &["name", "email", "url"];

    // Map JSON object keys to a compact enum so `visit_map` can dispatch quickly.
    enum Field {
      Name,
      Email,
      Url,
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
            formatter.write_str("`name`, `email` or `url`")
          }

          fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
          where
            E: de::Error,
          {
            Ok(match value {
              "name" => Field::Name,
              "email" => Field::Email,
              "url" => Field::Url,
              _ => Field::Ignore,
            })
          }
        }

        deserializer.deserialize_identifier(FieldVisitor)
      }
    }

    struct PersonObjectVisitor;

    impl<'de> Visitor<'de> for PersonObjectVisitor {
      type Value = PersonObject;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object with `name`, optional `email`, optional `url`")
      }

      fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        // `name` is required, `email/url` are optional.
        let mut name: Option<String> = None;
        let mut email: Option<String> = None;
        let mut url: Option<String> = None;
        // Track duplicates for optional fields as well, matching serde's strict behavior.
        let mut seen_email = false;
        let mut seen_url = false;

        while let Some(key) = map.next_key::<Field>()? {
          match key {
            Field::Name => {
              if name.is_some() {
                return Err(de::Error::duplicate_field("name"));
              }
              name = Some(map.next_value()?);
            }
            Field::Email => {
              if seen_email {
                return Err(de::Error::duplicate_field("email"));
              }
              email = map.next_value()?;
              seen_email = true;
            }
            Field::Url => {
              if seen_url {
                return Err(de::Error::duplicate_field("url"));
              }
              url = map.next_value()?;
              seen_url = true;
            }
            Field::Ignore => {
              // Explicitly consume unknown fields so they don't affect parsing.
              let _: IgnoredAny = map.next_value()?;
            }
          }
        }

        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;

        Ok(PersonObject { name, email, url })
      }
    }

    deserializer.deserialize_struct("PersonObject", FIELDS, PersonObjectVisitor)
  }
}

impl<'de> Deserialize<'de> for Person {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct PersonVisitor;

    impl<'de> Visitor<'de> for PersonVisitor {
      type Value = Person;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string or an object for person")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Person::String(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Person::String(value))
      }

      fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        // Reuse PersonObject's manual map parsing for the object branch.
        let person = PersonObject::deserialize(MapAccessDeserializer::new(map))?;
        Ok(Person::Object(person))
      }
    }

    // Accept both JSON string and JSON object for compatibility with npm's author format.
    deserializer.deserialize_any(PersonVisitor)
  }
}

impl Validator for Person {
  fn validate(&self, prop: Option<&ObjectProp>) -> miette::Result<()> {
    match self {
      Person::String(name) => {
        if !name.is_empty() {
          return Ok(());
        }

        Err(validation_error(
          "Invalid name",
          Some("invalid_name"),
          "Please provide a valid name",
          value_range(prop, &[]),
          "Invalid name",
        ))
      }
      Person::Object(person) => {
        if person.name.is_empty() {
          return Err(validation_error(
            "Invalid name",
            Some("invalid_name"),
            "Please provide a valid name",
            value_range(prop, &["name"]),
            "Invalid name",
          ));
        }

        if let Some(email) = person.email.as_ref() {
          if !email.validate_email() {
            return Err(validation_error(
              "Invalid email",
              Some("invalid_email"),
              "Please provide a valid email",
              value_range(prop, &["email"]),
              "Invalid email",
            ));
          }
        }

        if let Some(url) = person.url.as_ref() {
          if !url.validate_url() {
            return Err(validation_error(
              "Invalid URL",
              Some("invalid_url"),
              "Please provide a valid URL",
              value_range(prop, &["url"]),
              "Invalid URL",
            ));
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

  #[test]
  fn should_fail_parse_person_when_author_type_is_invalid() {
    let jsones = [
      r#"{ 
        "author": 123 
      }"#,
      r#"{ 
        "author": true 
      }"#,
      r#"{ 
        "author": ["test"] 
      }"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json);
      assert!(res.is_ok());
      let res = res.unwrap();
      assert!(res.author().is_err());
    }
  }

  #[test]
  fn should_deserialize_person_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"author":{ "name": "test" }}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_person_object_when_required_field_is_missing() {
    let parsed = PackageJsonParser::parse_str(r#"{"author":{ "email": "a@b.com" }}"#);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert!(parsed.author().is_err());
  }

  #[test]
  fn should_fail_deserialize_person_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
