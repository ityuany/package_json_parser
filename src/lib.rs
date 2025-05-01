pub use def::bin::*;
pub use def::bugs::*;
pub use def::directories::*;
pub use def::engines::*;
pub use def::license::*;
pub use def::name::*;
pub use def::package_manager::*;
pub use def::person::*;
pub use def::publish_config::*;
pub use def::repository::*;
pub use def::r#type::*;
pub use def::version::*;
pub use err::*;
pub use rustc_hash::FxHashMap;
pub use serde::{Deserialize, Serialize};
pub use serde_valid::Validate;
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::{
  fs::File,
  io::{BufReader, Error},
};

mod def;
mod err;
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

  #[serde(skip)]
  pub __raw_source: Option<String>,

  #[serde(skip)]
  pub __raw_path: Option<PathBuf>,
}

impl PackageJsonParser {
  pub fn parse<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
    let file = File::open(path.as_ref())?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    let mut package_json_parser: PackageJsonParser = serde_json::from_str(&content)?;
    package_json_parser.__raw_source = Some(content);
    package_json_parser.__raw_path = Some(path.as_ref().to_path_buf());
    Ok(package_json_parser)
  }
}

impl PackageJsonParser {
  pub fn bin_to_hash_map(&self) -> Result<HashMap<String, String>, ErrorKind> {
    let bin = self.bin.as_ref().unwrap();
    let bin = match bin {
      Bin::String(v) => {
        let mut map = HashMap::default();
        let name = self
          .name
          .as_ref()
          .and_then(|name| name.0.split("/").last())
          .ok_or(ErrorKind::NameRequired)?;

        map.insert(name.to_string(), v.to_string());
        map
      }
      Bin::Object(o) => o.to_owned(),
    };

    Ok(bin)
  }
}
