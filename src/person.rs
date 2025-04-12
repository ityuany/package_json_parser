use crate::validator::Validator;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Debug, Validate, Serialize, Deserialize)]
pub enum Person {
    String(String),

    Object(#[validate] PersonObject),
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PersonObject {
    pub name: String,
    #[validate(custom = Validator::use_option_email)]
    pub email: Option<String>,
    #[validate(custom = Validator::use_option_url)]
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_validate_person() {
        let person = Person::String("test".to_string());
        assert!(person.validate().is_ok());
    }

    #[test]
    fn should_pass_validate_person_object() {
        let person = Person::Object(PersonObject {
            name: "test".to_string(),
            email: Some("test@example.com".to_string()),
            url: Some("https://example.com".to_string()),
        });
        assert!(person.validate().is_ok());
    }

    #[test]
    fn should_fail_when_person_object_email_is_invalid() {
        let person = Person::Object(PersonObject {
            name: "test".to_string(),
            email: Some("invalid".to_string()),
            url: None,
        });
        let res = person.validate();
        assert!(res.is_err());
    }

    #[test]
    fn should_fail_when_person_object_url_is_invalid() {
        let person = Person::Object(PersonObject {
            name: "test".to_string(),
            email: None,
            url: Some("invalid".to_string()),
        });
        assert!(person.validate().is_err());
    }
}
