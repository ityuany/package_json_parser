use jsonc_parser::ast::ObjectProp;
use serde::de::{self, IgnoredAny, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Clone)]
pub struct Directories {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bin: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub lib: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub man: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub doc: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub example: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub test: Option<String>,
}

impl<'de> Deserialize<'de> for Directories {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    const FIELDS: &[&str] = &["bin", "lib", "man", "doc", "example", "test"];

    enum Field {
      Bin,
      Lib,
      Man,
      Doc,
      Example,
      Test,
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
            formatter.write_str("a directories field")
          }

          fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
          where
            E: de::Error,
          {
            Ok(match value {
              "bin" => Field::Bin,
              "lib" => Field::Lib,
              "man" => Field::Man,
              "doc" => Field::Doc,
              "example" => Field::Example,
              "test" => Field::Test,
              _ => Field::Ignore,
            })
          }
        }

        deserializer.deserialize_identifier(FieldVisitor)
      }
    }

    struct DirectoriesVisitor;

    impl<'de> Visitor<'de> for DirectoriesVisitor {
      type Value = Directories;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object for directories")
      }

      fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
      where
        M: MapAccess<'de>,
      {
        let mut bin = None;
        let mut lib = None;
        let mut man = None;
        let mut doc = None;
        let mut example = None;
        let mut test = None;
        let mut seen_bin = false;
        let mut seen_lib = false;
        let mut seen_man = false;
        let mut seen_doc = false;
        let mut seen_example = false;
        let mut seen_test = false;

        while let Some(key) = map.next_key::<Field>()? {
          match key {
            Field::Bin => {
              if seen_bin {
                return Err(de::Error::duplicate_field("bin"));
              }
              bin = map.next_value()?;
              seen_bin = true;
            }
            Field::Lib => {
              if seen_lib {
                return Err(de::Error::duplicate_field("lib"));
              }
              lib = map.next_value()?;
              seen_lib = true;
            }
            Field::Man => {
              if seen_man {
                return Err(de::Error::duplicate_field("man"));
              }
              man = map.next_value()?;
              seen_man = true;
            }
            Field::Doc => {
              if seen_doc {
                return Err(de::Error::duplicate_field("doc"));
              }
              doc = map.next_value()?;
              seen_doc = true;
            }
            Field::Example => {
              if seen_example {
                return Err(de::Error::duplicate_field("example"));
              }
              example = map.next_value()?;
              seen_example = true;
            }
            Field::Test => {
              if seen_test {
                return Err(de::Error::duplicate_field("test"));
              }
              test = map.next_value()?;
              seen_test = true;
            }
            Field::Ignore => {
              let _: IgnoredAny = map.next_value()?;
            }
          }
        }

        Ok(Directories {
          bin,
          lib,
          man,
          doc,
          example,
          test,
        })
      }
    }

    deserializer.deserialize_struct("Directories", FIELDS, DirectoriesVisitor)
  }
}

impl Validator for Directories {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_deserialize_directories_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"directories":{ "bin": "bin", "lib": "lib" }}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_directories_when_field_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"directories":{ "bin": true }}"#);
    assert!(parsed.is_err());
  }

  #[test]
  fn should_fail_deserialize_directories_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
