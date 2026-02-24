use jsonc_parser::ast::ObjectProp;
use serde::de::{self, IgnoredAny, MapAccess, Visitor, value::MapAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone)]
pub enum Readme {
  String(String),
  Object(ReadmeContent),
}

#[derive(Debug, Serialize, Clone)]
pub struct ReadmeContent {
  pub r#type: String,
  pub value: String,
}

impl<'de> Deserialize<'de> for ReadmeContent {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    const FIELDS: &[&str] = &["type", "value"];

    enum Field {
      Type,
      Value,
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
            formatter.write_str("`type` or `value`")
          }

          fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
          where
            E: de::Error,
          {
            Ok(match value {
              "type" => Field::Type,
              "value" => Field::Value,
              _ => Field::Ignore,
            })
          }
        }

        deserializer.deserialize_identifier(FieldVisitor)
      }
    }

    struct ReadmeContentVisitor;

    impl<'de> Visitor<'de> for ReadmeContentVisitor {
      type Value = ReadmeContent;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object with `type` and `value`")
      }

      fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        let mut r#type = None;
        let mut value = None;

        while let Some(key) = map.next_key::<Field>()? {
          match key {
            Field::Type => {
              if r#type.is_some() {
                return Err(de::Error::duplicate_field("type"));
              }
              r#type = Some(map.next_value()?);
            }
            Field::Value => {
              if value.is_some() {
                return Err(de::Error::duplicate_field("value"));
              }
              value = Some(map.next_value()?);
            }
            Field::Ignore => {
              let _: IgnoredAny = map.next_value()?;
            }
          }
        }

        let r#type = r#type.ok_or_else(|| de::Error::missing_field("type"))?;
        let value = value.ok_or_else(|| de::Error::missing_field("value"))?;

        Ok(ReadmeContent { r#type, value })
      }
    }

    deserializer.deserialize_struct("ReadmeContent", FIELDS, ReadmeContentVisitor)
  }
}

impl<'de> Deserialize<'de> for Readme {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ReadmeVisitor;

    impl<'de> Visitor<'de> for ReadmeVisitor {
      type Value = Readme;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string or an object for readme")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Readme::String(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Readme::String(value))
      }

      fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        let content = ReadmeContent::deserialize(MapAccessDeserializer::new(map))?;
        Ok(Readme::Object(content))
      }
    }

    deserializer.deserialize_any(ReadmeVisitor)
  }
}

impl Validator for Readme {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_deserialize_readme_successfully() {
    let parsed =
      PackageJsonParser::parse_str(r#"{"readme":{ "type": "text", "value": "README" }}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_readme_when_field_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"readme":{ "type": true, "value": "README" }}"#);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert!(parsed.readme().is_err());
  }

  #[test]
  fn should_fail_deserialize_readme_content_when_required_field_is_missing() {
    let parsed = PackageJsonParser::parse_str(r#"{"readme":{ "type": "text" }}"#);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert!(parsed.readme().is_err());
  }

  #[test]
  fn should_fail_deserialize_readme_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
