use jsonc_parser::ast::ObjectProp;
use serde::de::{self, IgnoredAny, MapAccess, Visitor, value::MapAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use validator::ValidateUrl;

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, Serialize, Clone)]
pub struct Repository {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub r#type: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub directory: Option<String>,
}

impl<'de> Deserialize<'de> for Repository {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    const FIELDS: &[&str] = &["type", "url", "directory"];

    enum Field {
      Type,
      Url,
      Directory,
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
            formatter.write_str("`type`, `url` or `directory`")
          }

          fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
          where
            E: de::Error,
          {
            Ok(match value {
              "type" => Field::Type,
              "url" => Field::Url,
              "directory" => Field::Directory,
              _ => Field::Ignore,
            })
          }
        }

        deserializer.deserialize_identifier(FieldVisitor)
      }
    }

    struct RepositoryVisitor;

    impl<'de> Visitor<'de> for RepositoryVisitor {
      type Value = Repository;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object for repository")
      }

      fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        let mut r#type = None;
        let mut url = None;
        let mut directory = None;
        let mut seen_type = false;
        let mut seen_url = false;
        let mut seen_directory = false;

        while let Some(key) = map.next_key::<Field>()? {
          match key {
            Field::Type => {
              if seen_type {
                return Err(de::Error::duplicate_field("type"));
              }
              r#type = map.next_value()?;
              seen_type = true;
            }
            Field::Url => {
              if seen_url {
                return Err(de::Error::duplicate_field("url"));
              }
              url = map.next_value()?;
              seen_url = true;
            }
            Field::Directory => {
              if seen_directory {
                return Err(de::Error::duplicate_field("directory"));
              }
              directory = map.next_value()?;
              seen_directory = true;
            }
            Field::Ignore => {
              let _: IgnoredAny = map.next_value()?;
            }
          }
        }

        Ok(Repository {
          r#type,
          url,
          directory,
        })
      }
    }

    deserializer.deserialize_struct("Repository", FIELDS, RepositoryVisitor)
  }
}

#[derive(Debug, Serialize, Clone)]
pub enum RepositoryOrString {
  Repository(Repository),
  String(String),
}

impl<'de> Deserialize<'de> for RepositoryOrString {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct RepositoryOrStringVisitor;

    impl<'de> Visitor<'de> for RepositoryOrStringVisitor {
      type Value = RepositoryOrString;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string or an object for repository")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(RepositoryOrString::String(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(RepositoryOrString::String(value))
      }

      fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        let repository = Repository::deserialize(MapAccessDeserializer::new(map))?;
        Ok(RepositoryOrString::Repository(repository))
      }
    }

    deserializer.deserialize_any(RepositoryOrStringVisitor)
  }
}

impl Validator for RepositoryOrString {
  fn validate(&self, repository: Option<&ObjectProp>) -> miette::Result<()> {
    match self {
      RepositoryOrString::Repository(repos) => {
        if let Some(url) = repos.url.as_ref() {
          if !url.validate_url() {
            return Err(validation_error(
              "Invalid url",
              Some("invalid_url"),
              "Please provide a valid url",
              value_range(repository, &["url"]),
              "Invalid url",
            ));
          }
        }
        Ok(())
      }
      RepositoryOrString::String(string) => {
        if !string.validate_url() {
          return Err(validation_error(
            "Invalid url",
            Some("invalid_url"),
            "Please provide a valid url",
            value_range(repository, &[]),
            "Invalid url",
          ));
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
  fn should_pass_validate_repository() {
    let jsones = [
      r#"{"repository": {"type": "git", "url": "https://github.com/rust-lang/rust", "directory": "src"}}"#,
      r#"{"repository": "https://github.com/rust-lang/rust"}"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_ok());
    }
  }

  #[test]
  fn should_fail_validate_repository() {
    let jsones = [
      r#"{"repository": {"type": "git", "url": "invalid", "directory": "src"}}"#,
      r#"{"repository": "invalid"}"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_err());
    }
  }
}
