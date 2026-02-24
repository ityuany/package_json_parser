use jsonc_parser::ast::ObjectProp;
use serde::de::{self, IgnoredAny, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use validator::ValidateUrl;

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct PublishConfig {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub access: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub registry: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tag: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub provenance: Option<bool>,
}

impl<'de> Deserialize<'de> for PublishConfig {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    const FIELDS: &[&str] = &["access", "registry", "tag", "provenance"];

    enum Field {
      Access,
      Registry,
      Tag,
      Provenance,
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
            formatter.write_str("a publishConfig field")
          }

          fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
          where
            E: de::Error,
          {
            Ok(match value {
              "access" => Field::Access,
              "registry" => Field::Registry,
              "tag" => Field::Tag,
              "provenance" => Field::Provenance,
              _ => Field::Ignore,
            })
          }
        }

        deserializer.deserialize_identifier(FieldVisitor)
      }
    }

    struct PublishConfigVisitor;

    impl<'de> Visitor<'de> for PublishConfigVisitor {
      type Value = PublishConfig;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object for publishConfig")
      }

      fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        let mut access = None;
        let mut registry = None;
        let mut tag = None;
        let mut provenance = None;
        let mut seen_access = false;
        let mut seen_registry = false;
        let mut seen_tag = false;
        let mut seen_provenance = false;

        while let Some(key) = map.next_key::<Field>()? {
          match key {
            Field::Access => {
              if seen_access {
                return Err(de::Error::duplicate_field("access"));
              }
              access = map.next_value()?;
              seen_access = true;
            }
            Field::Registry => {
              if seen_registry {
                return Err(de::Error::duplicate_field("registry"));
              }
              registry = map.next_value()?;
              seen_registry = true;
            }
            Field::Tag => {
              if seen_tag {
                return Err(de::Error::duplicate_field("tag"));
              }
              tag = map.next_value()?;
              seen_tag = true;
            }
            Field::Provenance => {
              if seen_provenance {
                return Err(de::Error::duplicate_field("provenance"));
              }
              provenance = map.next_value()?;
              seen_provenance = true;
            }
            Field::Ignore => {
              let _: IgnoredAny = map.next_value()?;
            }
          }
        }

        Ok(PublishConfig {
          access,
          registry,
          tag,
          provenance,
        })
      }
    }

    deserializer.deserialize_struct("PublishConfig", FIELDS, PublishConfigVisitor)
  }
}

impl Validator for PublishConfig {
  fn validate(&self, publish_config: Option<&ObjectProp>) -> miette::Result<()> {
    if let Some(access) = self.access.as_ref() {
      let access_regex = lazy_regex::regex_is_match!(r"^(public|restricted|private)$", access);
      if !access_regex {
        return Err(validation_error(
          "Invalid access",
          None,
          "Please provide a valid access",
          value_range(publish_config, &["access"]),
          "Invalid access",
        ));
      }
    }

    if let Some(registry) = self.registry.as_ref() {
      if !registry.validate_url() {
        return Err(validation_error(
          "Invalid registry",
          None,
          "Please provide a valid registry",
          value_range(publish_config, &["registry"]),
          "Invalid registry",
        ));
      }
    }

    if let Some(tag) = self.tag.as_ref() {
      let tag_regex = lazy_regex::regex_is_match!(r"^[a-zA-Z0-9-_.]+$", tag);
      if !tag_regex {
        return Err(validation_error(
          "Invalid tag",
          None,
          "Please provide a valid tag",
          value_range(publish_config, &["tag"]),
          "Invalid tag",
        ));
      }
    }

    if let Some(provenance) = self.provenance.as_ref() {
      if !provenance {
        return Err(validation_error(
          "Invalid provenance",
          None,
          "Please provide a valid provenance",
          value_range(publish_config, &["provenance"]),
          "Invalid provenance",
        ));
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_pass_validate_publish_config() {
    let jsones = [
      r#"{"publishConfig": {"access": "public", "registry": "https://registry.npmjs.org/"}}"#,
      r#"{"publishConfig": {"access": "restricted", "registry": "https://registry.npmjs.org/"}}"#,
      r#"{"publishConfig": {"access": "private", "registry": "https://registry.npmjs.org/"}}"#,
      r#"{"publishConfig": {"access": "public", "registry": "https://registry.npmjs.org/", "tag": "invalid"}}"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_ok());
    }
  }

  #[test]
  fn should_fail_validate_publish_config() {
    let jsones = [
      r#"{"publishConfig": {"access": "invalid", "registry": "https://registry.npmjs.org/"}}"#,
      r#"{"publishConfig": {"access": "public", "registry": "invalid"}}"#,
      r#"{"publishConfig": {"access": "invalid", "registry": "invalid"}}"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_err());
    }
  }

  #[test]
  fn should_deserialize_publish_config_successfully() {
    let parsed =
      PackageJsonParser::parse_str(r#"{"publishConfig":{ "access": "public", "tag": "latest" }}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_publish_config_when_field_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"publishConfig":{ "provenance": "true" }}"#);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert!(parsed.validate().is_err());
  }

  #[test]
  fn should_fail_deserialize_publish_config_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
