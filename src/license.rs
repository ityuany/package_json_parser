use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Debug, PartialEq, Validate, Deserialize, Serialize)]

pub struct License(
    #[validate(
        pattern = "^(AGPL-3.0-only|Apache-2.0|BSD-2-Clause|BSD-3-Clause|BSL-1.0|CC0-1.0|CDDL-1.0|CDDL-1.1|EPL-1.0|EPL-2.0|GPL-2.0-only|GPL-3.0-only|ISC|LGPL-2.0-only|LGPL-2.1-only|LGPL-2.1-or-later|LGPL-3.0-only|LGPL-3.0-or-later|MIT|MPL-2.0|MSPL|UnLicense)$"
    )]
    pub String,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_validate_license() {
        let license = License("AGPL-3.0-only".to_string());
        assert!(license.validate().is_ok());
    }

    #[test]
    fn should_fail_when_license_is_invalid() {
        let license = License("MIT1".to_string());
        assert!(license.validate().is_err());
    }
}
