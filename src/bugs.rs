use serde::{Deserialize, Serialize};
use serde_valid::Validate;

use crate::validator::Validator;

#[derive(Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct BugsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = Validator::use_option_url)]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = Validator::use_option_email)]
    pub email: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(untagged)]
pub enum Bugs {
    Url(#[validate(custom = Validator::use_url)] String),
    #[validate(custom = Validator::use_email)]
    Email(#[validate(custom = Validator::use_email)] String),
    #[validate]
    BugsItem(BugsItem),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_validate_bugs_item() {
        let bugs_item = BugsItem {
            url: Some("https://example.com".to_string()),
            email: Some("test@example.com".to_string()),
        };
        assert!(bugs_item.validate().is_ok());
    }

    #[test]
    fn should_fail_when_item_url_is_invalid() {
        let bugs_item = BugsItem {
            url: Some("invalid".to_string()),
            email: Some("test@example.com".to_string()),
        };
        assert!(bugs_item.validate().is_err());
    }

    #[test]
    fn should_fail_when_item_email_is_invalid() {
        let bugs_item = BugsItem {
            url: Some("https://example.com".to_string()),
            email: Some("invalid".to_string()),
        };
        assert!(bugs_item.validate().is_err());
    }

    #[test]
    fn should_pass_validate_bugs() {
        let bugs = Bugs::BugsItem(BugsItem {
            url: Some("https://example.com".to_string()),
            email: Some("test@example.com".to_string()),
        });
        assert!(bugs.validate().is_ok());
    }

    #[test]
    fn should_fail_when_url_is_invalid1() {
        let bugs = Bugs::Url("invalid".to_string());
        let res = bugs.validate();
        println!("{:?}", res);
        assert!(res.is_err());
    }

    #[test]
    fn should_fail_when_email_is_invalid() {
        let bugs = Bugs::Email("invalid".to_string());
        let res = bugs.validate();
        assert!(res.is_err());
    }

    #[test]
    fn should_pass_validate_bugs_item_when_url_is_none() {
        let bugs = Bugs::Url("https://example.com".to_string());
        assert!(bugs.validate().is_ok());
    }

    #[test]
    fn should_pass_validate_bugs_item_when_email_is_none() {
        let bugs = Bugs::Email("test@example.com".to_string());
        assert!(bugs.validate().is_ok());
    }
}
