use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, PartialEq, Serialize, Clone, Deref, DerefMut)]
pub struct Name(String);

impl<'de> Deserialize<'de> for Name {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct NameVisitor;

    impl<'de> Visitor<'de> for NameVisitor {
      type Value = Name;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string for name")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Name(value.to_string()))
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Name(value))
      }
    }

    deserializer.deserialize_any(NameVisitor)
  }
}

impl Name {
  pub fn get_bin_name(&self) -> &str {
    self.split("/").last().unwrap_or(self)
  }
}

impl Validator for Name {
  fn validate(&self, prop: Option<&ObjectProp>) -> miette::Result<()> {
    let reg_name = lazy_regex::regex_is_match!(
      r"^(?:(?:@(?:[a-z0-9-*~][a-z0-9-*._~]*)?/[a-z0-9-._~])|[a-z0-9-~])[a-z0-9-._~]*$",
      &self
    );

    if reg_name {
      return Ok(());
    }

    Err(validation_error(
      "Package name does not match required pattern",
      Some("invalid_package_name"),
      r"Expected pattern: ^(?:(?:@(?:[a-z0-9-*~][a-z0-9-*._~]*)?/[a-z0-9-._~])|[a-z0-9-~])[a-z0-9-._~]*$",
      value_range(prop, &[]),
      "here",
    ))
  }
}

#[cfg(test)]
mod tests {

  use crate::PackageJsonParser;

  #[test]
  fn should_pass_validate_name_with_regex() {
    let jsones = [r#"{"name": "test"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_ok());
    }
  }

  #[test]
  fn should_fail_validate_name_with_regex() {
    let jsones = [r#"{"name": "tesSSSt"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_err());
    }
  }
}
