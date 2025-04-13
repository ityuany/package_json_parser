use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Engines(HashMap<String, String>);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_pass_validate_engines() {
    let engines = Engines(HashMap::from([(
      "node".to_string(),
      ">=10.0.0".to_string(),
    )]));
    assert!(engines.validate().is_ok());
  }

  #[test]
  fn should_pass_deserialize_engines() {
    let raw = r#"{"node": ">=10.0.0"}"#;
    let engines: Engines = serde_json::from_str(raw).unwrap();
    assert!(engines.validate().is_ok());
  }

  #[test]
  fn should_pass_deserialize_engines_with_multiple_engines() {
    let raw = r#"{"node": ">=10.0.0", "npm": ">=6.0.0"}"#;
    let engines: Engines = serde_json::from_str(raw).unwrap();
    assert!(engines.validate().is_ok());
  }
}
