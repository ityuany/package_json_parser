use serde::{Deserialize, Serialize};

use crate::utils::{is_email, is_url};

#[derive(Debug)]
pub enum Person {
    String(String),
    Object(PersonObject),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonObject {
    pub name: String,
    pub email: Option<String>,
    pub url: Option<String>,
}

impl<'de> Deserialize<'de> for Person {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = serde_json::Value::deserialize(deserializer)?;
        if let serde_json::Value::String(value) = v {
            Ok(Person::String(value))
        } else if let serde_json::Value::Object(map) = v {
            let name = map
                .get("name")
                .and_then(|name| name.as_str())
                .map(|name| name.to_string())
                .ok_or_else(|| serde::de::Error::custom("Invalid person"))?;

            let email = map
                .get("email")
                .and_then(|email| email.as_str())
                .map(|email| {
                    if !is_email(email) {
                        return Err(serde::de::Error::custom("Invalid email"));
                    }
                    Ok(email.to_string())
                })
                .transpose()?;

            let url = map
                .get("url")
                .and_then(|url| url.as_str())
                .map(|url| {
                    if !is_url(url) {
                        return Err(serde::de::Error::custom("Invalid url"));
                    }
                    Ok(url.to_string())
                })
                .transpose()?;

            Ok(Person::Object(PersonObject { name, email, url }))
        } else {
            Err(serde::de::Error::custom("Invalid person"))
        }
    }
}

impl Serialize for Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Person::String(s) => serializer.serialize_str(s),
            Person::Object(o) => {
                use serde::ser::SerializeStruct;
                let mut state = serializer.serialize_struct("Person", 3)?;
                state.serialize_field("name", &o.name)?;
                state.serialize_field("email", &o.email)?;
                state.serialize_field("url", &o.url)?;
                state.end()
            }
        }
    }
}
