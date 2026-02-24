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
