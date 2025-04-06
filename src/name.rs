use lazy_regex::regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize)]
pub struct Name(String);

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;

        // 验证包名长度
        if name.len() > 214 {
            return Err(serde::de::Error::custom("Package name too long"));
        }

        let r = regex!(
            "^(?:(?:@(?:[a-z0-9-*~][a-z0-9-*._~]*)?/[a-z0-9-._~])|[a-z0-9-~])[a-z0-9-._~]*$"
        );

        if !r.is_match(&name) {
            return Err(serde::de::Error::custom(
                r#"package name does not match the pattern of "^(?:(?:@(?:[a-z0-9-*~][a-z0-9-*._~]*)?/[a-z0-9-._~])|[a-z0-9-~])[a-z0-9-._~]*$"."#,
            ));
        }
        Ok(Name(name))
    }
}
