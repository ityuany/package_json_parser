use lazy_regex::regex;

pub struct Validator;

impl Validator {
    pub fn is_url(v: &str) -> bool {
        let r = regex!(
            r"^https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&//=]*)$"
        );
        r.is_match(&v)
    }

    pub fn is_email(v: &str) -> bool {
        let r = regex!(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$");
        r.is_match(&v)
    }

    pub fn use_url(url: &String) -> Result<(), serde_valid::validation::Error> {
        if !Validator::is_url(url) {
            return Err(serde_valid::validation::Error::Custom(
                "registry must be a valid URL".to_string(),
            ));
        }

        Ok(())
    }

    pub fn use_email(email: &String) -> Result<(), serde_valid::validation::Error> {
        if !Validator::is_email(email) {
            return Err(serde_valid::validation::Error::Custom(
                "email must be a valid email".to_string(),
            ));
        }

        Ok(())
    }

    pub fn use_url_or_email(url_or_email: &String) -> Result<(), serde_valid::validation::Error> {
        if !Validator::is_url(url_or_email) && !Validator::is_email(url_or_email) {
            return Err(serde_valid::validation::Error::Custom(
                "url or email must be a valid url or email".to_string(),
            ));
        }

        Ok(())
    }

    pub fn use_option_url(url: &Option<String>) -> Result<(), serde_valid::validation::Error> {
        if let Some(url) = url {
            Validator::use_url(url)
        } else {
            Ok(())
        }
    }

    pub fn use_option_email(email: &Option<String>) -> Result<(), serde_valid::validation::Error> {
        if let Some(email) = email {
            Validator::use_email(email)
        } else {
            Ok(())
        }
    }
}
