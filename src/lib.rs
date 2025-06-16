pub use def::*;

pub use rustc_hash::FxHashMap;
pub use serde::{Deserialize, Serialize};
pub use serde_valid::Validate;
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::{fs::File, io::BufReader};

pub use crate::err::ErrorKind;
pub use miette::{LabeledSpan, NamedSource, Result, SourceSpan};

mod case;
mod def;
mod err;
mod ext;
mod validator;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJsonParser {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<Name>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub version: Option<Version>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<Description>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub keywords: Option<Keywords>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub homepage: Option<HomePage>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub bugs: Option<Bugs>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub license: Option<License>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub author: Option<Person>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub contributors: Option<Vec<Person>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub maintainers: Option<Vec<Person>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub files: Option<Vec<String>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub main: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub r#type: Option<Type>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub types: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub typings: Option<String>,

  #[serde(rename = "packageManager", skip_serializing_if = "Option::is_none")]
  pub package_manager: Option<PackageManager>,

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
  pub __raw_path: Option<String>,
}

impl PackageJsonParser {
  // pub fn v(&self) {
  //   if let Some(source) = self.__raw_source.as_ref() {
  //     let parse_result =
  //       parse_to_ast(source, &CollectOptions::default(), &ParseOptions::default()).unwrap();

  //     if let Some(name) = self.name.as_ref() {
  //       name.validate(&parse_result);
  //     }
  //   }
  // }

  pub fn parse<P: AsRef<Path>>(path: P) -> Result<Self> {
    let file = File::open(path.as_ref()).map_err(ErrorKind::IoError)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader
      .read_to_string(&mut content)
      .map_err(ErrorKind::IoError)?;
    let mut package_json_parser: PackageJsonParser =
      serde_json::from_str(&content).map_err(|e| {
        let line = e.line();
        let _col = e.column();

        let offset = content
          .lines()
          .take(line.saturating_sub(1))
          .map(|l| l.len() + 1)
          .sum::<usize>();

        let len = content
          .lines()
          .nth(line.saturating_sub(1))
          .unwrap_or("")
          .len();

        let primary_span = SourceSpan::from(0..content.len());

        let name_source = NamedSource::new(path.as_ref().to_str().unwrap(), content.clone());

        let err = ErrorKind::JsonParseError {
          src: name_source,
          primary_span: Some(primary_span),
          other_spans: vec![LabeledSpan::new(Some("here".to_string()), offset, len)],
          source: Some(e),
          advice: Some("Please check the JSON syntax".to_string()),
        };
        miette::miette!(err)
      })?;
    package_json_parser.__raw_source = Some(content);
    package_json_parser.__raw_path = Some(path.as_ref().to_string_lossy().to_string());
    Ok(package_json_parser)
  }
}

impl PackageJsonParser {
  pub fn bin_to_hash_map(&self) -> Result<HashMap<String, String>> {
    let bin = self.bin.as_ref().unwrap();
    let bin = match bin {
      Bin::String(v) => {
        let mut map = HashMap::default();
        let name = self
          .name
          .as_ref()
          .and_then(|name| name.0.split("/").last())
          .ok_or(miette::miette!(ErrorKind::NameRequired))?;

        map.insert(name.to_string(), v.to_string());
        map
      }
      Bin::Object(o) => o.to_owned(),
    };

    Ok(bin)
  }
}
