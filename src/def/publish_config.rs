use lazy_regex::regex;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

use crate::validator::Validator;

#[derive(Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PublishConfig {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(custom = validate_access)]
  pub access: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(custom = Validator::use_option_url)]
  pub registry: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tag: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub provenance: Option<bool>,
}

fn validate_access(access: &Option<String>) -> Result<(), serde_valid::validation::Error> {
  if let Some(ref access) = access {
    let r = regex!(r"^(public|restricted|private)$");
    if !r.is_match(access) {
      return Err(serde_valid::validation::Error::Custom(
        "access must be one of public, restricted, or private".to_string(),
      ));
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_pass_validate_publish_config() {
    let publish_config = PublishConfig {
      access: Some("public".to_string()),
      registry: Some("https://registry.npmjs.org/".to_string()),
      tag: Some("latest".to_string()),
      provenance: Some(true),
    };
    assert!(publish_config.validate().is_ok());
  }

  #[test]
  fn should_pass_validate_publish_config_when_access_is_none() {
    let publish_config = PublishConfig {
      access: None,
      registry: None,
      tag: None,
      provenance: None,
    };
    assert!(publish_config.validate().is_ok());
  }

  #[test]
  fn should_pass_validate_publish_config_when_registry_is_none() {
    let publish_config = PublishConfig {
      access: Some("public".to_string()),
      registry: None,
      tag: Some("latest".to_string()),
      provenance: Some(true),
    };
    assert!(publish_config.validate().is_ok());
  }

  #[test]
  fn should_fail_when_access_is_invalid() {
    let publish_config = PublishConfig {
      access: Some("invalid".to_string()),
      registry: None,
      tag: None,
      provenance: None,
    };
    let res = publish_config.validate();
    if let Err(e) = res {
      println!("{:#?}", e.to_string());
    }
    assert!(publish_config.validate().is_err());
  }

  #[test]
  fn should_fail_when_registry_is_invalid() {
    let publish_config = PublishConfig {
      access: None,
      registry: Some("invalid".to_string()),
      tag: None,
      provenance: None,
    };
    assert!(publish_config.validate().is_err());
  }

  #[test]
  fn should_fail_when_provenance_is_invalid() {
    let publish_config = PublishConfig {
      access: None,
      registry: None,
      tag: None,
      provenance: Some(true),
    };
    assert!(publish_config.validate().is_ok());
  }
}
