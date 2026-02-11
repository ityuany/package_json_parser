use derive_more::{Deref, DerefMut};
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};

use crate::ext::{Validator, validation_error, value_range};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Deref, DerefMut)]
pub struct License(String);

impl Validator for License {
  fn validate(&self, prop: Option<&ObjectProp>) -> miette::Result<()> {
    let regex = lazy_regex::regex_is_match!(
      r"^(AGPL-3.0-only|Apache-2.0|BSD-2-Clause|BSD-3-Clause|BSL-1.0|CC0-1.0|CDDL-1.0|CDDL-1.1|EPL-1.0|EPL-2.0|GPL-2.0-only|GPL-3.0-only|ISC|LGPL-2.0-only|LGPL-2.1-only|LGPL-2.1-or-later|LGPL-3.0-only|LGPL-3.0-or-later|MIT|MPL-2.0|MSPL|UnLicense)$",
      &self
    );

    if regex {
      return Ok(());
    }

    Err(validation_error(
      "Invalid license",
      Some("invalid_license"),
      "Please provide a valid license",
      value_range(prop, &[]),
      "here",
    ))
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_pass_validate_license() {
    let jsones = [
      r#"{"license": "MIT"}"#,
      r#"{"license": "Apache-2.0"}"#,
      r#"{"license": "BSD-2-Clause"}"#,
      r#"{"license": "BSD-3-Clause"}"#,
    ];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_ok());
    }
  }

  #[test]
  fn should_fail_when_license_is_invalid() {
    let jsones = [r#"{"license": "MIT1"}"#];

    for json in jsones {
      let res = PackageJsonParser::parse_str(json).unwrap();
      let res = res.validate();
      assert!(res.is_err());
    }
  }
}
