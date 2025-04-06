use bugs::Bugs;
use license::License;
use name::Name;
use person::Person;
use r#type::Type;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, Error},
};
use version::Version;

mod bugs;
mod license;
mod name;
mod person;
mod r#type;
mod utils;
mod version;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJsonParser {
    pub name: Option<Name>,
    pub version: Option<Version>,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub homepage: Option<String>,
    pub bugs: Option<Bugs>,
    pub license: Option<License>,
    pub author: Option<Person>,
    pub contributors: Option<Vec<Person>>,
    pub maintainers: Option<Vec<Person>>,
    pub files: Option<Vec<String>>,
    pub main: Option<String>,
    pub types: Option<String>,
    pub r#type: Option<Type>,
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
    // pub os: Option<Vec<String>>,
    // pub cpu: Option<Vec<String>>,

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
    use super::*;

    #[test]
    fn test_name() {
        let package_json_parser = PackageJsonParser::parse("fixtures/name-package.json");
        assert!(package_json_parser.is_err());
    }

    #[test]
    fn test_version() {
        let package_json_parser = PackageJsonParser::parse("fixtures/version-package.json");
        assert!(package_json_parser.is_err());
    }

    #[test]
    fn test_bugs() {
        let package_json_parser = PackageJsonParser::parse("fixtures/bugs-url-string.json");
        assert!(package_json_parser.is_err());
    }

    #[test]
    fn test_bugs_object() {
        let package_json_parser = PackageJsonParser::parse("fixtures/bugs-object.json");
        assert!(package_json_parser.is_ok());
    }

    #[test]
    fn test_license() {
        let package_json_parser = PackageJsonParser::parse("fixtures/license-apache-2.0.json");
        assert!(package_json_parser.is_ok());
        assert_eq!(
            package_json_parser.unwrap().license,
            Some(License::Apache20)
        );
    }

    #[test]
    fn test_author() {
        let package_json_parser = PackageJsonParser::parse("fixtures/author-object.json");
        assert!(package_json_parser.is_ok());
    }

    #[test]
    fn test_author_string() {
        let package_json_parser = PackageJsonParser::parse("fixtures/author-string.json");
        assert!(package_json_parser.is_ok());
    }

    #[test]
    fn test_type() {
        let package_json_parser = PackageJsonParser::parse("fixtures/type-module.json");
        assert!(package_json_parser.is_ok());
        assert_eq!(package_json_parser.unwrap().r#type, Some(Type::Module));
        let package_json_parser = PackageJsonParser::parse("fixtures/type-commonjs.json");
        assert!(package_json_parser.is_ok());
        assert_eq!(package_json_parser.unwrap().r#type, Some(Type::Commonjs));
    }
}
