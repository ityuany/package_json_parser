use jsonc_parser::{ast::ObjectProp, common::Ranged};
use miette::{LabeledSpan, MietteDiagnostic, Severity};
use serde::{Deserialize, Serialize};

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

impl Validator for PublishConfig {
  fn validate(&self, publish_config: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    let mut diagnostics = vec![];

    if let Some(access) = self.access.as_ref() {
      let access_regex = lazy_regex::regex_is_match!(r"^(public|restricted|private)$", access);
      if !access_regex {
        let mut labels = vec![];

        if let Some(range) = publish_config
          .and_then(|prop| prop.value.as_object())
          .and_then(|obj| obj.get("access"))
          .and_then(|prop| prop.value.as_string_lit())
          .map(|value| value.range())
          .and_then(|range| Some(range.start..range.end))
        {
          labels.push(LabeledSpan::at(range, "Invalid access"));
        }
        let diagnostic = MietteDiagnostic::new("Invalid access".to_string())
          .with_labels(labels)
          .with_severity(Severity::Error)
          .with_help("Please provide a valid access");
        diagnostics.push(diagnostic);
      }
    }

    if let Some(registry) = self.registry.as_ref() {
      let registry_regex = lazy_regex::regex_is_match!(
        r"^https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&//=]*)$",
        registry
      );
      if !registry_regex {
        let mut labels = vec![];
        if let Some(range) = publish_config
          .and_then(|prop| prop.value.as_object())
          .and_then(|obj| obj.get("registry"))
          .and_then(|prop| prop.value.as_string_lit())
          .map(|value| value.range())
          .and_then(|range| Some(range.start..range.end))
        {
          labels.push(LabeledSpan::at(range, "Invalid registry"));
        }
        let diagnostic = MietteDiagnostic::new("Invalid registry".to_string())
          .with_labels(labels)
          .with_severity(Severity::Error)
          .with_help("Please provide a valid registry");
        diagnostics.push(diagnostic);
      }
    }
    return diagnostics;
  }
}

#[cfg(test)]
mod tests {
  use crate::case::t;

  use super::*;

  #[test]
  fn should_pass_validate_publish_config() {
    let jsones = [
      r#"{"publishConfig": {"access": "public", "registry": "https://registry.npmjs.org/"}}"#,
      r#"{"publishConfig": {"access": "restricted", "registry": "https://registry.npmjs.org/"}}"#,
      r#"{"publishConfig": {"access": "private", "registry": "https://registry.npmjs.org/"}}"#,
      r#"{"publishConfig": {"access": "public", "registry": "https://registry.npmjs.org/", "tag": "invalid"}}"#,
    ];

    t(&jsones, |parser, parse_result| {
      let publish_config = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|v| v.get("publishConfig"));
      let res = parser.publish_config.unwrap().validate(publish_config);
      assert!(res.is_empty());
      res
    });
  }

  #[test]
  fn should_fail_validate_publish_config() {
    let jsones = [
      r#"{"publishConfig": {"access": "invalid", "registry": "https://registry.npmjs.org/"}}"#,
      r#"{"publishConfig": {"access": "public", "registry": "invalid"}}"#,
      r#"{"publishConfig": {"access": "invalid", "registry": "invalid"}}"#,
    ];

    t(&jsones, |parser, parse_result| {
      let publish_config = parse_result
        .value
        .as_ref()
        .and_then(|v| v.as_object())
        .and_then(|v| v.get("publishConfig"));
      let res = parser.publish_config.unwrap().validate(publish_config);
      assert!(!res.is_empty());
      res
    });
  }

  // #[test]
  // fn should_pass_validate_publish_config() {
  //   let publish_config = PublishConfig {
  //     access: Some("public".to_string()),
  //     registry: Some("https://registry.npmjs.org/".to_string()),
  //     tag: Some("latest".to_string()),
  //     provenance: Some(true),
  //   };
  //   assert!(publish_config.validate().is_ok());
  // }

  // #[test]
  // fn should_pass_validate_publish_config_when_access_is_none() {
  //   let publish_config = PublishConfig {
  //     access: None,
  //     registry: None,
  //     tag: None,
  //     provenance: None,
  //   };
  //   assert!(publish_config.validate().is_ok());
  // }

  // #[test]
  // fn should_pass_validate_publish_config_when_registry_is_none() {
  //   let publish_config = PublishConfig {
  //     access: Some("public".to_string()),
  //     registry: None,
  //     tag: Some("latest".to_string()),
  //     provenance: Some(true),
  //   };
  //   assert!(publish_config.validate().is_ok());
  // }

  // #[test]
  // fn should_fail_when_access_is_invalid() {
  //   let publish_config = PublishConfig {
  //     access: Some("invalid".to_string()),
  //     registry: None,
  //     tag: None,
  //     provenance: None,
  //   };
  //   let res = publish_config.validate();
  //   if let Err(e) = res {
  //     println!("{:#?}", e.to_string());
  //   }
  //   assert!(publish_config.validate().is_err());
  // }

  // #[test]
  // fn should_fail_when_registry_is_invalid() {
  //   let publish_config = PublishConfig {
  //     access: None,
  //     registry: Some("invalid".to_string()),
  //     tag: None,
  //     provenance: None,
  //   };
  //   assert!(publish_config.validate().is_err());
  // }

  // #[test]
  // fn should_fail_when_provenance_is_invalid() {
  //   let publish_config = PublishConfig {
  //     access: None,
  //     registry: None,
  //     tag: None,
  //     provenance: Some(true),
  //   };
  //   assert!(publish_config.validate().is_ok());
  // }
}
