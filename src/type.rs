use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Commonjs,
    Module,
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Type::Commonjs => serializer.serialize_str("commonjs"),
            Type::Module => serializer.serialize_str("module"),
        }
    }
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "commonjs" => Ok(Type::Commonjs),
            "module" => Ok(Type::Module),
            _ => Err(serde::de::Error::custom("Invalid type")),
        }
    }
}
