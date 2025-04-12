use bin::Bin;
use bugs::Bugs;
use directories::Directories;
use engines::Engines;
use license::License;
use name::Name;
use package_manager::PackageManager;
use person::Person;
use publish_config::PublishConfig;
use r#type::Type;
use repository::RepositoryOrString;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Error},
};
use version::Version;

mod bin;
mod bugs;
mod directories;
mod engines;
mod license;
mod name;
mod package_manager;
mod person;
mod publish_config;
mod repository;
mod r#type;
mod utils;
mod validator;
mod version;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PackageJsonParser {
    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,

    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,

    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bugs: Option<Bugs>,

    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,

    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Person>,

    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<Person>>,

    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintainers: Option<Vec<Person>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<String>,

    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub typings: Option<String>,

    #[validate]
    #[serde(rename = "packageManager", skip_serializing_if = "Option::is_none")]
    pub package_manager: Option<PackageManager>,

    #[validate]
    #[serde(rename = "publishConfig", skip_serializing_if = "Option::is_none")]
    pub publish_config: Option<PublishConfig>,
    // pub browser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<Bin>,
    // pub typings: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub man: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Directories>,

    #[validate]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RepositoryOrString>,

    // pub funding: Option<Funding>,
    // pub config: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines: Option<Engines>,

    #[serde(rename = "engineStrict", skip_serializing_if = "Option::is_none")]
    pub engine_strict: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<FxHashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<FxHashMap<String, String>>,

    #[serde(rename = "devDependencies", skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<FxHashMap<String, String>>,

    #[serde(
        rename = "optionalDependencies",
        skip_serializing_if = "Option::is_none"
    )]
    pub optional_dependencies: Option<FxHashMap<String, String>>,

    #[serde(rename = "peerDependencies", skip_serializing_if = "Option::is_none")]
    pub peer_dependencies: Option<FxHashMap<String, String>>,
    // #[serde(rename = "peerDependenciesMeta")]
    // pub peer_dependencies_meta: Option<FxHashMap<String, String>>,
    // pub overrides: Option<FxHashMap<String, OverrideValue>>,
}

impl PackageJsonParser {
    pub fn parse(path: &str) -> Result<Self, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let package_json_parser = serde_json::from_reader(reader)?;
        Ok(package_json_parser)
    }
}

#[cfg(test)]
mod tests {
    use serde_valid::Validate;

    use super::*;

    #[test]
    fn should_pass_validate_package_json_parser() {
        let json = r#"{"bin": {"test": "test.js"}}"#;
        let package_json_parser = serde_json::from_str::<PackageJsonParser>(json).unwrap();

        println!("{:#?}", package_json_parser);

        let r = package_json_parser.validate();

        println!("{:#?}", r);

        assert!(r.is_ok());
    }
}
