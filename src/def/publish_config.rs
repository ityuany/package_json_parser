use std::ops::Range;

use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};
use validator::ValidateUrl;

use crate::ext::Validator;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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

impl PublishConfig {
  fn get_publish_config_access_range(
    &self,
    publish_config: Option<&ObjectProp>,
  ) -> Option<Range<usize>> {
    publish_config
      .and_then(|prop| prop.value.as_object())
      .and_then(|obj| obj.get("access"))
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }

  fn get_publish_config_registry_range(
    &self,
    publish_config: Option<&ObjectProp>,
  ) -> Option<Range<usize>> {
    publish_config
      .and_then(|prop| prop.value.as_object())
      .and_then(|obj| obj.get("registry"))
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }

  fn get_publish_config_tag_range(
    &self,
    publish_config: Option<&ObjectProp>,
  ) -> Option<Range<usize>> {
    publish_config
      .and_then(|prop| prop.value.as_object())
      .and_then(|obj| obj.get("tag"))
      .and_then(|prop| prop.value.as_string_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }

  fn get_publish_config_provenance_range(
    &self,
    publish_config: Option<&ObjectProp>,
  ) -> Option<Range<usize>> {
    publish_config
      .and_then(|prop| prop.value.as_object())
      .and_then(|obj| obj.get("provenance"))
      .and_then(|prop| prop.value.as_boolean_lit())
      .map(|value| value.range())
      .map(|range| range.start..range.end)
  }
}

impl Validator for PublishConfig {
  fn validate(&self, publish_config: Option<&ObjectProp>) -> miette::Result<()> {
    if let Some(access) = self.access.as_ref() {
      let access_regex = lazy_regex::regex_is_match!(r"^(public|restricted|private)$", access);
      if !access_regex {
        let mut labels = vec![];

        if let Some(range) = self.get_publish_config_access_range(publish_config) {
          labels.push(LabeledSpan::at(range, "Invalid access"));
        }
        let diagnostic = MietteDiagnostic::new("Invalid access".to_string())
          .with_labels(labels)
          .with_severity(Severity::Error)
          .with_help("Please provide a valid access");
        return Err(miette::miette!(diagnostic));
      }
    }

    if let Some(registry) = self.registry.as_ref() {
      if !registry.validate_url() {
        let mut labels = vec![];
        if let Some(range) = self.get_publish_config_registry_range(publish_config) {
          labels.push(LabeledSpan::at(range, "Invalid registry"));
        }
        let diagnostic = MietteDiagnostic::new("Invalid registry".to_string())
          .with_labels(labels)
          .with_severity(Severity::Error)
          .with_help("Please provide a valid registry");
        return Err(miette::miette!(diagnostic));
      }
    }

    if let Some(tag) = self.tag.as_ref() {
      let tag_regex = lazy_regex::regex_is_match!(r"^[a-zA-Z0-9-_.]+$", tag);
      if !tag_regex {
        let mut labels = vec![];
        if let Some(range) = self.get_publish_config_tag_range(publish_config) {
          labels.push(LabeledSpan::at(range, "Invalid tag"));
        }
        let diagnostic = MietteDiagnostic::new("Invalid tag".to_string())
          .with_labels(labels)
          .with_severity(Severity::Error)
          .with_help("Please provide a valid tag");
        return Err(miette::miette!(diagnostic));
      }
    }

    if let Some(provenance) = self.provenance.as_ref() {
      if !provenance {
        let mut labels = vec![];
        if let Some(range) = self.get_publish_config_provenance_range(publish_config) {
          labels.push(LabeledSpan::at(range, "Invalid provenance"));
        }
        let diagnostic = MietteDiagnostic::new("Invalid provenance".to_string())
          .with_labels(labels)
          .with_severity(Severity::Error)
          .with_help("Please provide a valid provenance");
        return Err(miette::miette!(diagnostic));
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
}
