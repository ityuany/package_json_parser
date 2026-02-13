pub use def::*;
use miette::{MietteDiagnostic, Severity};

use crate::err::{JsonParseError, PackageJsonError};
use crate::ext::{Validator, value_range};
use jsonc_parser::{CollectOptions, ParseOptions, parse_to_ast};
pub use rustc_hash::FxHashMap;
use serde::de::DeserializeOwned;
pub use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::{fs::File, io::BufReader};

pub use crate::err::{ErrorKind, Result};
pub use access::*;
pub use miette::{LabeledSpan, NamedSource, SourceSpan};
pub use validation::*;

mod access;
mod def;
mod err;
mod ext;
mod validation;

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

  #[serde(skip)]
  pub __raw_json: Option<Value>,
}

impl PackageJsonParser {
  fn from_raw_json(raw_json: Value, source: String, path: Option<String>) -> Self {
    fn decode_optional<T: DeserializeOwned>(raw_json: &Value, key: &str) -> Option<T> {
      raw_json
        .get(key)
        .and_then(|raw| serde_json::from_value::<T>(raw.clone()).ok())
    }

    Self {
      name: decode_optional(&raw_json, "name"),
      version: decode_optional(&raw_json, "version"),
      description: decode_optional(&raw_json, "description"),
      keywords: decode_optional(&raw_json, "keywords"),
      homepage: decode_optional(&raw_json, "homepage"),
      bugs: decode_optional(&raw_json, "bugs"),
      license: decode_optional(&raw_json, "license"),
      author: decode_optional(&raw_json, "author"),
      contributors: decode_optional(&raw_json, "contributors"),
      maintainers: decode_optional(&raw_json, "maintainers"),
      files: decode_optional(&raw_json, "files"),
      main: decode_optional(&raw_json, "main"),
      r#type: decode_optional(&raw_json, "type"),
      types: decode_optional(&raw_json, "types"),
      typings: decode_optional(&raw_json, "typings"),
      package_manager: decode_optional(&raw_json, "packageManager"),
      publish_config: decode_optional(&raw_json, "publishConfig"),
      bin: decode_optional(&raw_json, "bin"),
      man: decode_optional(&raw_json, "man"),
      directories: decode_optional(&raw_json, "directories"),
      repository: decode_optional(&raw_json, "repository"),
      module: decode_optional(&raw_json, "module"),
      readme: decode_optional(&raw_json, "readme"),
      private: decode_optional(&raw_json, "private"),
      engines: decode_optional(&raw_json, "engines"),
      engine_strict: decode_optional(&raw_json, "engineStrict"),
      os: decode_optional(&raw_json, "os"),
      cpu: decode_optional(&raw_json, "cpu"),
      scripts: decode_optional(&raw_json, "scripts"),
      dependencies: decode_optional(&raw_json, "dependencies"),
      dev_dependencies: decode_optional(&raw_json, "devDependencies"),
      optional_dependencies: decode_optional(&raw_json, "optionalDependencies"),
      peer_dependencies: decode_optional(&raw_json, "peerDependencies"),
      __raw_source: Some(source),
      __raw_path: path,
      __raw_json: Some(raw_json),
    }
  }

  fn raw_source(&self) -> Result<&str> {
    self.__raw_source.as_deref().ok_or_else(|| {
      PackageJsonError::InternalState(
        "raw source is not available, was this parsed correctly?".to_string(),
      )
    })
  }

  pub(crate) fn raw_json(&self) -> Result<&Value> {
    self.__raw_json.as_ref().ok_or_else(|| {
      PackageJsonError::InternalState(
        "raw json is not available, was this parsed correctly?".to_string(),
      )
    })
  }

  fn with_source(&self, e: miette::Report) -> Result<miette::Report> {
    let source = self.raw_source()?.to_string();
    if let Some(path) = self.__raw_path.as_ref() {
      return Ok(e.with_source_code(NamedSource::new(path, source)));
    }
    Ok(e.with_source_code(source))
  }

  pub fn validate(&self) -> Result<ValidationReport> {
    let Ok(parse_result) = parse_to_ast(
      self.raw_source()?,
      &CollectOptions::default(),
      &ParseOptions::default(),
    ) else {
      let labeled_span = LabeledSpan::new(Some("here".to_string()), 0, 0);
      let diagnostic = MietteDiagnostic::new("Failed to parse JSON")
        .with_label(labeled_span)
        .with_severity(Severity::Error);
      let report = miette::miette!(diagnostic);
      return Err(PackageJsonError::Validation(self.with_source(report)?));
    };

    let root = parse_result.value.as_ref().and_then(|v| v.as_object());
    let mut report = ValidationReport::default();

    macro_rules! collect_field_issues {
      ($ty:ty, $field:expr) => {
        report.push_many(self.collect_field_issues::<$ty>($field, root)?);
      };
    }

    collect_field_issues!(Name, ValidationField::Name);
    collect_field_issues!(Version, ValidationField::Version);
    collect_field_issues!(Description, ValidationField::Description);
    collect_field_issues!(Keywords, ValidationField::Keywords);
    collect_field_issues!(HomePage, ValidationField::Homepage);
    collect_field_issues!(Bugs, ValidationField::Bugs);
    collect_field_issues!(License, ValidationField::License);
    collect_field_issues!(Person, ValidationField::Author);
    collect_field_issues!(Contributors, ValidationField::Contributors);
    collect_field_issues!(Maintainers, ValidationField::Maintainers);
    collect_field_issues!(Files, ValidationField::Files);
    collect_field_issues!(Main, ValidationField::Main);
    collect_field_issues!(Type, ValidationField::Type);
    collect_field_issues!(Types, ValidationField::Types);
    collect_field_issues!(Typings, ValidationField::Typings);
    collect_field_issues!(PackageManager, ValidationField::PackageManager);
    collect_field_issues!(PublishConfig, ValidationField::PublishConfig);
    collect_field_issues!(Bin, ValidationField::Bin);
    collect_field_issues!(Man, ValidationField::Man);
    collect_field_issues!(Directories, ValidationField::Directories);
    collect_field_issues!(RepositoryOrString, ValidationField::Repository);
    collect_field_issues!(Module, ValidationField::Module);
    collect_field_issues!(Readme, ValidationField::Readme);
    collect_field_issues!(Private, ValidationField::Private);
    collect_field_issues!(Engines, ValidationField::Engines);
    collect_field_issues!(EngineStrict, ValidationField::EngineStrict);
    collect_field_issues!(Os, ValidationField::Os);
    collect_field_issues!(Cpu, ValidationField::Cpu);
    collect_field_issues!(Scripts, ValidationField::Scripts);
    collect_field_issues!(Dependencies, ValidationField::Dependencies);
    collect_field_issues!(DevDependencies, ValidationField::DevDependencies);
    collect_field_issues!(OptionalDependencies, ValidationField::OptionalDependencies);
    collect_field_issues!(PeerDependencies, ValidationField::PeerDependencies);

    Ok(report)
  }

  pub(crate) fn expected_help(field: ValidationField) -> Option<&'static str> {
    match field {
      ValidationField::Main => Some("`main` only supports `bool` or `string`."),
      ValidationField::Bin => {
        Some("`bin` only supports `string` or object map `{ \"name\": \"path\" }`.")
      }
      ValidationField::Bugs => Some(
        "`bugs` only supports `string` (url/email) or object `{ \"url\"?: string, \"email\"?: string }`.",
      ),
      ValidationField::Repository => Some(
        "`repository` only supports `string` or object `{ \"type\"?: string, \"url\"?: string, \"directory\"?: string }`.",
      ),
      ValidationField::Keywords => Some("`keywords` only supports `string` or `string[]`."),
      ValidationField::Readme => {
        Some("`readme` only supports `string` or object `{ \"type\": string, \"value\": string }`.")
      }
      ValidationField::Author | ValidationField::Contributors | ValidationField::Maintainers => {
        Some(
          "person fields only support `string` or object `{ \"name\": string, \"email\"?: string, \"url\"?: string }`.",
        )
      }
      _ => None,
    }
  }

  pub(crate) fn collect_field_issues<T: DeserializeOwned + Validator>(
    &self,
    field: ValidationField,
    root: Option<&jsonc_parser::ast::Object>,
  ) -> Result<Vec<ValidationIssue>> {
    let key = field.json_key();
    let Some(raw) = self.raw_json()?.get(key).cloned() else {
      return Ok(Vec::new());
    };

    let prop = root.and_then(|obj| obj.get(key));
    let severity = ValidationSeverity::Error;

    match serde_json::from_value::<T>(raw) {
      Ok(value) => Ok(
        value
          .validate(prop)
          .into_iter()
          .map(|violation| ValidationIssue::from_violation(field, severity, violation))
          .collect(),
      ),
      Err(error) => Ok(vec![ValidationIssue::type_mismatch(
        field,
        severity,
        error.to_string(),
        Self::expected_help(field).map(ToString::to_string),
        value_range(prop, &[]),
      )]),
    }
  }

  fn build_parse_error<S: miette::SourceCode + std::fmt::Debug + 'static>(
    src: S,
    content: &str,
    error: serde_json::Error,
  ) -> miette::Report {
    let line = error.line();
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
    let advice = match error.classify() {
      serde_json::error::Category::Data => {
        "Data type/value does not match package.json field schema.".to_string()
      }
      serde_json::error::Category::Syntax | serde_json::error::Category::Eof => {
        "Please check the JSON syntax".to_string()
      }
      serde_json::error::Category::Io => "I/O error while reading JSON".to_string(),
    };

    miette::miette!(JsonParseError {
      src,
      primary_span: Some(SourceSpan::from(0..content.len())),
      other_spans: vec![LabeledSpan::new(Some("here".to_string()), offset, len)],
      source: Some(error),
      advice: Some(advice),
    })
  }

  pub fn parse_str(content: &str) -> Result<Self> {
    let raw_json = serde_json::from_str(content).map_err(|e| {
      PackageJsonError::JsonParse(Self::build_parse_error(content.to_string(), content, e))
    })?;
    Ok(Self::from_raw_json(raw_json, content.to_string(), None))
  }

  pub fn parse<P: AsRef<Path>>(path: P) -> Result<Self> {
    let file = File::open(path.as_ref()).map_err(PackageJsonError::Io)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader
      .read_to_string(&mut content)
      .map_err(PackageJsonError::Io)?;
    let raw_json = serde_json::from_str(&content).map_err(|e| {
      let src = NamedSource::new(path.as_ref().to_string_lossy(), content.clone());
      PackageJsonError::JsonParse(Self::build_parse_error(src, &content, e))
    })?;
    Ok(Self::from_raw_json(
      raw_json,
      content,
      Some(path.as_ref().to_string_lossy().to_string()),
    ))
  }
}

impl TryFrom<&Path> for PackageJsonParser {
  type Error = PackageJsonError;

  fn try_from(value: &Path) -> std::result::Result<Self, Self::Error> {
    Self::parse(value)
  }
}

impl TryFrom<&str> for PackageJsonParser {
  type Error = PackageJsonError;

  fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
    Self::parse_str(value)
  }
}

impl PackageJsonParser {
  pub fn bin_to_hash_map(&self) -> Result<HashMap<String, String>> {
    let bin = self.get_bin();
    let Some(bin) = bin.value else {
      return Ok(HashMap::default());
    };

    let bin = match bin {
      Bin::String(v) => {
        let mut map = HashMap::default();
        let name = self.get_name();
        let Some(name) = name.value else {
          return Err(PackageJsonError::NameRequired);
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
    let report = j.validate().unwrap();
    assert!(report.has_errors());
    assert!(report.warnings.is_empty());
  }
}
