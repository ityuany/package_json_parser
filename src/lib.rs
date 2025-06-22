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

    if let Some(name) = self.name.as_ref() {
      let name_json = root.and_then(|obj| obj.get("name"));
      self.handle_error(name.validate(name_json))?;
    }

    if let Some(version) = self.version.as_ref() {
      let version_json = root.and_then(|obj| obj.get("version"));
      self.handle_error(version.validate(version_json))?;
    }

    if let Some(description) = self.description.as_ref() {
      let description_json = root.and_then(|obj| obj.get("description"));
      self.handle_error(description.validate(description_json))?;
    }

    if let Some(keywords) = self.keywords.as_ref() {
      let keywords_json = root.and_then(|obj| obj.get("keywords"));
      self.handle_error(keywords.validate(keywords_json))?;
    }

    if let Some(homepage) = self.homepage.as_ref() {
      let homepage_json = root.and_then(|obj| obj.get("homepage"));
      self.handle_error(homepage.validate(homepage_json))?;
    }

    if let Some(bugs) = self.bugs.as_ref() {
      let bugs_json = root.and_then(|obj| obj.get("bugs"));
      self.handle_error(bugs.validate(bugs_json))?;
    }

    if let Some(license) = self.license.as_ref() {
      let license_json = root.and_then(|obj| obj.get("license"));
      self.handle_error(license.validate(license_json))?;
    }

    if let Some(author) = self.author.as_ref() {
      let author_json = root.and_then(|obj| obj.get("author"));
      self.handle_error(author.validate(author_json))?;
    }

    if let Some(contributors) = self.contributors.as_ref() {
      let contributors_json = root.and_then(|obj| obj.get("contributors"));
      self.handle_error(contributors.validate(contributors_json))?;
    }

    if let Some(maintainers) = self.maintainers.as_ref() {
      let maintainers_json = root.and_then(|obj| obj.get("maintainers"));
      self.handle_error(maintainers.validate(maintainers_json))?;
    }

    if let Some(files) = self.files.as_ref() {
      let files_json = root.and_then(|obj| obj.get("files"));
      self.handle_error(files.validate(files_json))?;
    }

    if let Some(main) = self.main.as_ref() {
      let main_json = root.and_then(|obj| obj.get("main"));
      self.handle_error(main.validate(main_json))?;
    }

    if let Some(r#type) = self.r#type.as_ref() {
      let r#type_json = root.and_then(|obj| obj.get("type"));
      self.handle_error(r#type.validate(r#type_json))?;
    }

    if let Some(types) = self.types.as_ref() {
      let types_json = root.and_then(|obj| obj.get("types"));
      self.handle_error(types.validate(types_json))?;
    }

    if let Some(typings) = self.typings.as_ref() {
      let typings_json = root.and_then(|obj| obj.get("typings"));
      self.handle_error(typings.validate(typings_json))?;
    }

    if let Some(package_manager) = self.package_manager.as_ref() {
      let package_manager_json = root.and_then(|obj| obj.get("packageManager"));
      self.handle_error(package_manager.validate(package_manager_json))?;
    }

    if let Some(publish_config) = self.publish_config.as_ref() {
      let publish_config_json = root.and_then(|obj| obj.get("publishConfig"));
      self.handle_error(publish_config.validate(publish_config_json))?;
    }

    if let Some(bin) = self.bin.as_ref() {
      let bin_json = root.and_then(|obj| obj.get("bin"));
      self.handle_error(bin.validate(bin_json))?;
    }

    if let Some(man) = self.man.as_ref() {
      let man_json = root.and_then(|obj| obj.get("man"));
      self.handle_error(man.validate(man_json))?;
    }

    if let Some(directories) = self.directories.as_ref() {
      let directories_json = root.and_then(|obj| obj.get("directories"));
      self.handle_error(directories.validate(directories_json))?;
    }

    if let Some(repository) = self.repository.as_ref() {
      let repository_json = root.and_then(|obj| obj.get("repository"));
      self.handle_error(repository.validate(repository_json))?;
    }

    if let Some(module) = self.module.as_ref() {
      let module_json = root.and_then(|obj| obj.get("module"));
      self.handle_error(module.validate(module_json))?;
    }

    if let Some(readme) = self.readme.as_ref() {
      let readme_json = root.and_then(|obj| obj.get("readme"));
      self.handle_error(readme.validate(readme_json))?;
    }

    if let Some(private) = self.private.as_ref() {
      let private_json = root.and_then(|obj| obj.get("private"));
      self.handle_error(private.validate(private_json))?;
    }

    if let Some(engines) = self.engines.as_ref() {
      let engines_json = root.and_then(|obj| obj.get("engines"));
      self.handle_error(engines.validate(engines_json))?;
    }

    if let Some(engine_strict) = self.engine_strict.as_ref() {
      let engine_strict_json = root.and_then(|obj| obj.get("engineStrict"));
      self.handle_error(engine_strict.validate(engine_strict_json))?;
    }

    if let Some(os) = self.os.as_ref() {
      let os_json = root.and_then(|obj| obj.get("os"));
      self.handle_error(os.validate(os_json))?;
    }

    if let Some(cpu) = self.cpu.as_ref() {
      let cpu_json = root.and_then(|obj| obj.get("cpu"));
      self.handle_error(cpu.validate(cpu_json))?;
    }

    if let Some(scripts) = self.scripts.as_ref() {
      let scripts_json = root.and_then(|obj| obj.get("scripts"));
      self.handle_error(scripts.validate(scripts_json))?;
    }

    if let Some(dependencies) = self.dependencies.as_ref() {
      let dependencies_json = root.and_then(|obj| obj.get("dependencies"));
      self.handle_error(dependencies.validate(dependencies_json))?;
    }

    if let Some(dev_dependencies) = self.dev_dependencies.as_ref() {
      let dev_dependencies_json = root.and_then(|obj| obj.get("devDependencies"));
      self.handle_error(dev_dependencies.validate(dev_dependencies_json))?;
    }

    if let Some(optional_dependencies) = self.optional_dependencies.as_ref() {
      let optional_dependencies_json = root.and_then(|obj| obj.get("optionalDependencies"));
      self.handle_error(optional_dependencies.validate(optional_dependencies_json))?;
    }

    if let Some(peer_dependencies) = self.peer_dependencies.as_ref() {
      let peer_dependencies_json = root.and_then(|obj| obj.get("peerDependencies"));
      self.handle_error(peer_dependencies.validate(peer_dependencies_json))?;
    }

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
    let bin = self.bin.as_ref().unwrap();
    let bin = match bin {
      Bin::String(v) => {
        let mut map = HashMap::default();
        let name = self
          .name
          .as_ref()
          .and_then(|name| name.split("/").last())
          .ok_or(miette::miette!(ErrorKind::NameRequired))?;

        map.insert(name.to_string(), v.to_string());
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
