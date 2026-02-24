use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Deserializer, Serialize};

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, PartialEq, Eq, Serialize, Clone, Deref, DerefMut)]
pub struct Type(String);

impl<'de> Deserialize<'de> for Type {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    String::deserialize(deserializer).map(Self)
  }
}

impl Validator for Type {
  fn validate(&self, prop: Option<&ObjectProp>) -> miette::Result<()> {
    let regex = lazy_regex::regex_is_match!(r"^(commonjs|module)$", &self);

    if regex {
      return Ok(());
    }

    Err(validation_error(
      "Invalid type",
      Some("invalid_type"),
      "Please provide a valid type",
      value_range(prop, &[]),
      "here",
    ))
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_pass_validate_type() {
    let jsones = [r#"{"type": "commonjs"}"#, r#"{"type": "module"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_ok());
    }
  }

  #[test]
  fn should_fail_validate_type() {
    let jsones = [r#"{"type": "invalid"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_err());
    }
  }
}
