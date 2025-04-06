use serde::{Deserialize, Serialize};

use crate::utils::{is_email, is_url};

#[derive(Debug, PartialEq, Serialize)]
pub struct BugsItem {
    pub url: Option<String>,
    pub email: Option<String>,
}

impl<'de> Deserialize<'de> for BugsItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = serde_json::Value::deserialize(deserializer)?;
        if let serde_json::Value::Object(v) = v {
            let url = v
                .get("url")
                .and_then(|v| v.as_str())
                .map(String::from)
                .map(|url| {
                    if !is_url(&url) {
                        Err(serde::de::Error::custom("Invalid url"))
                    } else {
                        Ok(url)
                    }
                })
                .transpose()?;
            let email = v
                .get("email")
                .and_then(|v| v.as_str())
                .map(String::from)
                .map(|email| {
                    if !is_email(&email) {
                        Err(serde::de::Error::custom("Invalid email"))
                    } else {
                        Ok(email)
                    }
                })
                .transpose()?;
            Ok(BugsItem { url, email })
        } else {
            Err(serde::de::Error::custom("Invalid bugs"))
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Bugs {
    Url(String),
    Email(String),
    BugsItem(BugsItem),
}

impl<'de> Deserialize<'de> for Bugs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        match value {
            serde_json::Value::String(v) if is_url(&v) => Ok(Bugs::Url(v)),
            serde_json::Value::String(v) if is_email(&v) => Ok(Bugs::Email(v)),
            serde_json::Value::Object(map) => {
                let item = serde_json::from_value(serde_json::Value::Object(map))
                    .map_err(serde::de::Error::custom)?;
                Ok(Bugs::BugsItem(item))
            }
            _ => Err(serde::de::Error::custom("Invalid bugs")),
        }
    }
}
