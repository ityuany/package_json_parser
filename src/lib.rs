pub use def::*;
use miette::{MietteDiagnostic, Severity};

use crate::err::{JsonFileParseError, JsonStrParseError};
use crate::ext::Validator;
use jsonc_parser::{CollectOptions, ParseOptions, parse_to_ast};
pub use rustc_hash::FxHashMap;
pub use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::{fs::File, io::BufReader};

pub use crate::err::ErrorKind;
pub use miette::{LabeledSpan, NamedSource, Result, SourceSpan};

mod def;
mod err;
mod ext;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
  pub contributors: Option<Contributors>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub maintainers: Option<Maintainers>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub files: Option<Files>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub main: Option<Main>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub r#type: Option<Type>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub types: Option<Types>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub typings: Option<Typings>,

  #[serde(rename = "packageManager", skip_serializing_if = "Option::is_none")]
  pub package_manager: Option<PackageManager>,

  #[serde(rename = "publishConfig", skip_serializing_if = "Option::is_none")]
  pub publish_config: Option<PublishConfig>,
  // // pub browser: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bin: Option<Bin>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub man: Option<Man>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub directories: Option<Directories>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub repository: Option<RepositoryOrString>,

  // // pub funding: Option<Funding>,
  // // pub config: Option<HashMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub module: Option<Module>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub readme: Option<Readme>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub private: Option<Private>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub engines: Option<Engines>,

  #[serde(rename = "engineStrict", skip_serializing_if = "Option::is_none")]
  pub engine_strict: Option<EngineStrict>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub os: Option<Os>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub cpu: Option<Cpu>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub scripts: Option<Scripts>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub dependencies: Option<Dependencies>,

  #[serde(rename = "devDependencies", skip_serializing_if = "Option::is_none")]
  pub dev_dependencies: Option<DevDependencies>,

  #[serde(
    rename = "optionalDependencies",
    skip_serializing_if = "Option::is_none"
  )]
  pub optional_dependencies: Option<OptionalDependencies>,

  #[serde(rename = "peerDependencies", skip_serializing_if = "Option::is_none")]
  pub peer_dependencies: Option<PeerDependencies>,

  #[serde(skip)]
  pub __raw_source: Option<String>,

  #[serde(skip)]
  pub __raw_path: Option<String>,
}

impl PackageJsonParser {
  fn handle_error(&self, e: miette::Result<()>) -> miette::Result<()> {
    if let Err(e) = e {
      if let Some(path) = self.__raw_path.as_ref() {
        let name_source = NamedSource::new(path, self.__raw_source.as_ref().unwrap().clone());
        return Err(e.with_source_code(name_source));
      }
      return Err(e.with_source_code(self.__raw_source.as_ref().unwrap().clone()));
    }
    Ok(())
  }

  pub fn validate(&self) -> miette::Result<()> {
    let Ok(parse_result) = parse_to_ast(
      self.__raw_source.as_ref().unwrap(),
      &CollectOptions::default(),
      &ParseOptions::default(),
    ) else {
      let labeled_span = LabeledSpan::new(Some("here".to_string()), 0, 0);
      let diagnostic = MietteDiagnostic::new("Failed to parse JSON")
        .with_label(labeled_span)
        .with_severity(Severity::Error);
      return Err(miette::miette!(diagnostic));
    };

    let root = parse_result.value.as_ref().and_then(|v| v.as_object());

    macro_rules! validate_field {
      ($field:ident, $json_key:expr) => {
        if let Some(ref val) = self.$field {
          let prop = root.and_then(|obj| obj.get($json_key));
          self.handle_error(val.validate(prop))?;
        }
      };
      ($field:ident) => {
        validate_field!($field, stringify!($field));
      };
    }

    validate_field!(name);
    validate_field!(version);
    validate_field!(description);
    validate_field!(keywords);
    validate_field!(homepage);
    validate_field!(bugs);
    validate_field!(license);
    validate_field!(author);
    validate_field!(contributors);
    validate_field!(maintainers);
    validate_field!(files);
    validate_field!(main);
    validate_field!(r#type, "type");
    validate_field!(types);
    validate_field!(typings);
    validate_field!(package_manager, "packageManager");
    validate_field!(publish_config, "publishConfig");
    validate_field!(bin);
    validate_field!(man);
    validate_field!(directories);
    validate_field!(repository);
    validate_field!(module);
    validate_field!(readme);
    validate_field!(private);
    validate_field!(engines);
    validate_field!(engine_strict, "engineStrict");
    validate_field!(os);
    validate_field!(cpu);
    validate_field!(scripts);
    validate_field!(dependencies);
    validate_field!(dev_dependencies, "devDependencies");
    validate_field!(optional_dependencies, "optionalDependencies");
    validate_field!(peer_dependencies, "peerDependencies");

    Ok(())
  }

  pub fn parse_str(content: &str) -> Result<Self> {
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

        let err = JsonStrParseError {
          src: content.to_string(),
          primary_span: Some(primary_span),
          other_spans: vec![LabeledSpan::new(Some("here".to_string()), offset, len)],
          source: Some(e),
          advice: Some("Please check the JSON syntax".to_string()),
        };
        miette::miette!(err)
      })?;
    package_json_parser.__raw_source = Some(content.to_string());
    Ok(package_json_parser)
  }

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

        let err = JsonFileParseError {
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

impl TryFrom<&Path> for PackageJsonParser {
  type Error = miette::ErrReport;

  fn try_from(value: &Path) -> Result<Self, Self::Error> {
    Self::parse(value)
  }
}

impl TryFrom<&str> for PackageJsonParser {
  type Error = miette::ErrReport;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Self::parse_str(value)
  }
}

impl PackageJsonParser {
  pub fn bin_to_hash_map(&self) -> Result<HashMap<String, String>> {
    let Some(bin) = &self.bin else {
      return Ok(HashMap::default());
    };

    let bin = match bin {
      Bin::String(v) => {
        let mut map = HashMap::default();
        let Some(name) = &self.name else {
          return Err(miette::miette!(ErrorKind::NameRequired));
        };
        let bin_name = name.get_bin_name();
        map.insert(bin_name.to_string(), v.to_string());
        map
      }
      Bin::Object(o) => o.to_owned(),
    };

    Ok(bin)
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn test_validate() {
    let jsones = [r#"
      {

        "private": true,
        "bugs": "222https://example.com"
      }"#];
    let j = PackageJsonParser::parse_str(jsones[0]).unwrap();
    let res = j.validate();
    assert!(res.is_err());
    // println!("{:?}", j);
  }
}
