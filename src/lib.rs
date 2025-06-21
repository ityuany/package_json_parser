pub use def::*;

use crate::err::{JsonFileParseError, JsonStrParseError};
use crate::ext::Validator;
use jsonc_parser::{CollectOptions, ParseOptions, parse_to_ast};
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
  // pub browser: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bin: Option<Bin>,
  // pub typings: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub man: Option<Man>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub directories: Option<Directories>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub repository: Option<RepositoryOrString>,

  // pub funding: Option<Funding>,
  // pub config: Option<HashMap<String, String>>,
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

  pub fn validate(&self) {
    let mut diagnostics = vec![];

    let parse_result = parse_to_ast(
      self.__raw_source.as_ref().unwrap(),
      &CollectOptions::default(),
      &ParseOptions::default(),
    )
    .unwrap();

    let root = parse_result.value.as_ref().and_then(|v| v.as_object());

    if let Some(name) = self.name.as_ref() {
      let name_json = root.and_then(|obj| obj.get("name"));
      diagnostics.extend(name.validate(name_json));
    }

    if let Some(version) = self.version.as_ref() {
      let version_json = root.and_then(|obj| obj.get("version"));
      diagnostics.extend(version.validate(version_json));
    }

    if let Some(description) = self.description.as_ref() {
      let description_json = root.and_then(|obj| obj.get("description"));
      diagnostics.extend(description.validate(description_json));
    }

    if let Some(keywords) = self.keywords.as_ref() {
      let keywords_json = root.and_then(|obj| obj.get("keywords"));
      diagnostics.extend(keywords.validate(keywords_json));
    }

    if let Some(homepage) = self.homepage.as_ref() {
      let homepage_json = root.and_then(|obj| obj.get("homepage"));
      diagnostics.extend(homepage.validate(homepage_json));
    }

    if let Some(bugs) = self.bugs.as_ref() {
      let bugs_json = root.and_then(|obj| obj.get("bugs"));
      diagnostics.extend(bugs.validate(bugs_json));
    }

    if let Some(license) = self.license.as_ref() {
      let license_json = root.and_then(|obj| obj.get("license"));
      diagnostics.extend(license.validate(license_json));
    }

    if let Some(author) = self.author.as_ref() {
      let author_json = root.and_then(|obj| obj.get("author"));
      diagnostics.extend(author.validate(author_json));
    }

    if let Some(contributors) = self.contributors.as_ref() {
      let contributors_json = root.and_then(|obj| obj.get("contributors"));
      diagnostics.extend(contributors.validate(contributors_json));
    }

    if let Some(maintainers) = self.maintainers.as_ref() {
      let maintainers_json = root.and_then(|obj| obj.get("maintainers"));
      diagnostics.extend(maintainers.validate(maintainers_json));
    }

    if let Some(files) = self.files.as_ref() {
      let files_json = root.and_then(|obj| obj.get("files"));
      diagnostics.extend(files.validate(files_json));
    }

    if let Some(main) = self.main.as_ref() {
      let main_json = root.and_then(|obj| obj.get("main"));
      diagnostics.extend(main.validate(main_json));
    }

    if let Some(r#type) = self.r#type.as_ref() {
      let r#type_json = root.and_then(|obj| obj.get("type"));
      diagnostics.extend(r#type.validate(r#type_json));
    }

    if let Some(types) = self.types.as_ref() {
      let types_json = root.and_then(|obj| obj.get("types"));
      diagnostics.extend(types.validate(types_json));
    }

    if let Some(typings) = self.typings.as_ref() {
      let typings_json = root.and_then(|obj| obj.get("typings"));
      diagnostics.extend(typings.validate(typings_json));
    }

    if let Some(package_manager) = self.package_manager.as_ref() {
      let package_manager_json = root.and_then(|obj| obj.get("packageManager"));
      diagnostics.extend(package_manager.validate(package_manager_json));
    }

    if let Some(publish_config) = self.publish_config.as_ref() {
      let publish_config_json = root.and_then(|obj| obj.get("publishConfig"));
      diagnostics.extend(publish_config.validate(publish_config_json));
    }

    if let Some(bin) = self.bin.as_ref() {
      let bin_json = root.and_then(|obj| obj.get("bin"));
      diagnostics.extend(bin.validate(bin_json));
    }

    if let Some(man) = self.man.as_ref() {
      let man_json = root.and_then(|obj| obj.get("man"));
      diagnostics.extend(man.validate(man_json));
    }

    if let Some(directories) = self.directories.as_ref() {
      let directories_json = root.and_then(|obj| obj.get("directories"));
      diagnostics.extend(directories.validate(directories_json));
    }

    if let Some(repository) = self.repository.as_ref() {
      let repository_json = root.and_then(|obj| obj.get("repository"));
      diagnostics.extend(repository.validate(repository_json));
    }

    if let Some(module) = self.module.as_ref() {
      let module_json = root.and_then(|obj| obj.get("module"));
      diagnostics.extend(module.validate(module_json));
    }

    if let Some(readme) = self.readme.as_ref() {
      let readme_json = root.and_then(|obj| obj.get("readme"));
      diagnostics.extend(readme.validate(readme_json));
    }

    if let Some(private) = self.private.as_ref() {
      let private_json = root.and_then(|obj| obj.get("private"));
      diagnostics.extend(private.validate(private_json));
    }

    if let Some(engines) = self.engines.as_ref() {
      let engines_json = root.and_then(|obj| obj.get("engines"));
      diagnostics.extend(engines.validate(engines_json));
    }

    if let Some(engine_strict) = self.engine_strict.as_ref() {
      let engine_strict_json = root.and_then(|obj| obj.get("engineStrict"));
      diagnostics.extend(engine_strict.validate(engine_strict_json));
    }

    if let Some(os) = self.os.as_ref() {
      let os_json = root.and_then(|obj| obj.get("os"));
      diagnostics.extend(os.validate(os_json));
    }

    if let Some(cpu) = self.cpu.as_ref() {
      let cpu_json = root.and_then(|obj| obj.get("cpu"));
      diagnostics.extend(cpu.validate(cpu_json));
    }

    if let Some(scripts) = self.scripts.as_ref() {
      let scripts_json = root.and_then(|obj| obj.get("scripts"));
      diagnostics.extend(scripts.validate(scripts_json));
    }

    if !diagnostics.is_empty() {
      println!("validate success");
      for diagnostic in diagnostics {
        let report = miette::Report::from(diagnostic)
          .with_source_code(self.__raw_source.as_ref().unwrap().clone());
        println!("{:?}", report);
      }
    }

    // diagnostics
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

        "private": true
      }"#];
    let j = PackageJsonParser::parse_str(jsones[0]).unwrap();
    println!("{:?}", j.private);
  }
}
