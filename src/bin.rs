use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(untagged)]
pub enum Bin {
    String(String),
    Object(HashMap<String, String>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_validate_bin() {
        let bin = Bin::String("test".to_string());
        assert!(bin.validate().is_ok());
    }

    #[test]
    fn should_pass_validate_bin_object() {
        let bin = Bin::Object(HashMap::from([("test".to_string(), "test".to_string())]));
        assert!(bin.validate().is_ok());
    }

    #[test]
    fn should_success_when_deserialize_bin_object() {
        let raw = r#"{"test": "test.js", "invalid": "invalid.js"}"#;
        let bin: Bin = serde_json::from_str(raw).unwrap();
        assert!(bin.validate().is_ok());
    }

    #[test]
    fn should_fail_when_deserialize_bin_string() {
        let raw = r#""test.js""#;
        let bin: Bin = serde_json::from_str(raw).unwrap();
        assert!(bin.validate().is_ok());
    }
}
