use crate::ext::Validator;
use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Clone, Deref, DerefMut)]
pub struct HomePage(String);

impl<'de> Deserialize<'de> for HomePage {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct HomePageVisitor;

    impl<'de> Visitor<'de> for HomePageVisitor {
      type Value = HomePage;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string for homepage")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(HomePage(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(HomePage(value))
      }
    }

    deserializer.deserialize_any(HomePageVisitor)
  }
}

impl Validator for HomePage {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_deserialize_home_page_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"homepage":"https://example.com"}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_home_page_when_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"homepage":123}"#);
    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert!(parsed.validate().is_err());
  }

  #[test]
  fn should_fail_deserialize_home_page_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
  }
}
