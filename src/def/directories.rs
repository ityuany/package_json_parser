use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Directories {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lib: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub man: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_validate_directories() {
        let raw = r#"{"bin": "bin", "lib": "lib", "man": "man", "doc": "doc", "example": "example", "test": "test"}"#;
        let directories: Directories = serde_json::from_str(raw).unwrap();
        assert!(directories.validate().is_ok());
    }

    #[test]
    fn should_pass_deserialize_directories() {
        let raw = r#"{"bin": "bin", "lib": "lib", "man": "man"}"#;
        let directories: Directories = serde_json::from_str(raw).unwrap();
        assert!(directories.validate().is_ok());
    }
}
