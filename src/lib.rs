pub use def::bin::*;
pub use def::bugs::*;
pub use def::directories::*;
pub use def::engines::*;
pub use def::license::*;
pub use def::name::*;
pub use def::package_manager::*;
pub use def::person::*;
pub use def::publish_config::*;
pub use def::r#type::*;
pub use def::repository::*;
pub use def::version::*;
pub use rustc_hash::FxHashMap;
pub use serde::{Deserialize, Serialize};
pub use serde_valid::Validate;
use std::{
    fs::File,
    io::{BufReader, Error},
};

mod def;
mod ext;
mod validator;

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
    use super::*;

    #[test]
    fn should_pass_validate_package_json_parser() {
        let raw = r#"
        {
            "name": "test"
        }
    "#;
        let res = serde_json::from_str::<PackageJsonParser>(raw);

        assert!(res.is_ok());

        if let Ok(package_json_parser) = res {
            assert_eq!(package_json_parser.name, Some(Name("test".to_string())));

            package_json_parser.validate();
        }
    }
}
