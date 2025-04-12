use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct Version(
    #[validate(
        pattern = r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"
    )]
    pub String,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_validate_version() {
        let version = Version("1.0.0".to_string());
        assert!(version.validate().is_ok());
    }

    #[test]
    fn should_pass_validate_version_with_prerelease() {
        let version = Version("1.0.0-alpha.1".to_string());
        assert!(version.validate().is_ok());
    }

    #[test]
    fn should_pass_validate_version_with_build() {
        let version = Version("1.0.0+build.1".to_string());
        assert!(version.validate().is_ok());
    }

    #[test]
    fn should_pass_validate_version_with_prerelease_and_build() {
        let version = Version("1.0.0-alpha.1+build.1".to_string());
        assert!(version.validate().is_ok());
    }
}
