use bugs::Bugs;
use license::License;
use name::Name;
use package_manager::PackageManager;
use person::Person;
use publish_config::PublishConfig;
use r#type::Type;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::{
    fs::File,
    io::{BufReader, Error},
};
use version::Version;

mod bugs;
mod license;
mod name;
mod package_manager;
mod person;
mod publish_config;
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
    // pub bin: Option<Bin>,
    // pub typings: Option<String>,
    // pub man: Option<Man>,
    // pub directories: Option<Directories>,
    // pub repository: Option<Repository>,
    // pub funding: Option<Funding>,
    // pub config: Option<Config>,
    // #[serde(rename = "publishConfig")]
    // pub publish_config: Option<PublishConfig>,
    // pub module: Option<String>,
    // pub readme: Option<String>,
    // pub private: Option<bool>,
    // pub engines: Option<Engines>,
    #[serde(rename = "engineStrict", skip_serializing_if = "Option::is_none")]
    pub engine_strict: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Vec<String>>,
    // typesVersions
    // pub scripts: Option<Scripts>,
    // pub dependencies: Option<FxHashMap<String, String>>,
    // #[serde(rename = "devDependencies")]
    // pub dev_dependencies: Option<FxHashMap<String, String>>,
    // #[serde(rename = "optionalDependencies")]
    // pub optional_dependencies: Option<FxHashMap<String, String>>,
    // #[serde(rename = "peerDependencies")]
    // pub peer_dependencies: Option<FxHashMap<String, String>>,
    // #[serde(rename = "peerDependenciesMeta")]
    // pub peer_dependencies_meta: Option<FxHashMap<String, String>>,
    // pub overrides: Option<FxHashMap<String, OverrideValue>>,
    // #[serde(rename = "packageManager")]
    // pub package_manager: Option<String>,
    //bundle_dependencies
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
        let json = r#"{"packageManager": "npm2@1.0.0"}"#;
        let package_json_parser = serde_json::from_str::<PackageJsonParser>(json).unwrap();

        println!("{:#?}", package_json_parser);

        let r = package_json_parser.validate();

        println!("{:#?}", r);

        assert!(r.is_ok());
    }
}
