use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Debug, PartialEq, Eq, Validate, Serialize, Deserialize)]
pub struct Type(#[validate(pattern = "^(commonjs|module)$")] pub String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_validate_type() {
        let type_ = Type("commonjs".to_string());
        assert!(type_.validate().is_ok());
    }

    #[test]
    fn should_fail_when_type_is_invalid() {
        let type_ = Type("invalid".to_string());
        assert!(type_.validate().is_err());
    }

    #[test]
    fn should_pass_serialize_type() {
        let type_ = Type("commonjs".to_string());
        let serialized = serde_json::to_string(&type_).unwrap();
        assert_eq!(serialized, "\"commonjs\"");
    }

    #[test]
    fn should_pass_deserialize_type() {
        let type_ = Type("commonjs".to_string());
        let deserialized = serde_json::from_str::<Type>(r#""commonjs""#).unwrap();
        assert_eq!(deserialized, type_);
    }
}
