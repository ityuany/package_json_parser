use crate::ext::Validator;
use derive_more::{Deref, DerefMut};
use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Deref, DerefMut)]
pub struct License(String);

impl License {
  fn get_license_range(&self, prop: Option<&ObjectProp>) -> Option<Range<usize>> {
    prop
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }
}

impl Validator for License {
  fn validate(&self, prop: Option<&ObjectProp>) -> miette::Result<()> {
    let regex = lazy_regex::regex_is_match!(
      r"^(AGPL-3.0-only|Apache-2.0|BSD-2-Clause|BSD-3-Clause|BSL-1.0|CC0-1.0|CDDL-1.0|CDDL-1.1|EPL-1.0|EPL-2.0|GPL-2.0-only|GPL-3.0-only|ISC|LGPL-2.0-only|LGPL-2.1-only|LGPL-2.1-or-later|LGPL-3.0-only|LGPL-3.0-or-later|MIT|MPL-2.0|MSPL|UnLicense)$",
      &self
    );

    if regex {
      return Ok(());
    }

    let range = self.get_license_range(prop);

    let mut diagnostic = MietteDiagnostic::new("Invalid license".to_string())
      .with_help("Please provide a valid license")
      .with_severity(Severity::Error)
      .with_code("invalid_license");

    if let Some(range) = range {
      let label = LabeledSpan::at(range, "here".to_string());
      diagnostic = diagnostic.with_labels(vec![label]);
    }

    return Err(miette::miette!(diagnostic));
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
