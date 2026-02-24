pub use def::*;
use miette::{MietteDiagnostic, Severity};
use serde::de::DeserializeOwned;

use crate::err::JsonParseError;
use crate::ext::Validator;
use jsonc_parser::{CollectOptions, ParseOptions, ast::ObjectProp, parse_to_ast};
pub use rustc_hash::FxHashMap;
pub use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::{fs::File, io::BufReader};

pub use crate::err::ErrorKind;
pub use miette::{LabeledSpan, NamedSource, Result, SourceSpan};

mod def;
mod err;
mod ext;

type FieldResult<T> = std::result::Result<Option<T>, FieldError>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum FieldErrorKind {
  Deserialize,
  Validation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FieldError {
  field: &'static str,
  kind: FieldErrorKind,
  message: String,
}

impl FieldError {
  fn deserialize(field: &'static str, error: serde_json::Error) -> Self {
    Self {
      field,
      kind: FieldErrorKind::Deserialize,
      message: error.to_string(),
    }
  }

  fn validation(field: &'static str, error: miette::Report) -> Self {
    Self {
      field,
      kind: FieldErrorKind::Validation,
      message: error.to_string(),
    }
  }

  fn message(&self) -> String {
    match self.kind {
      FieldErrorKind::Deserialize => {
        format!(
          "Failed to deserialize field `{}`: {}",
          self.field, self.message
        )
      }
      FieldErrorKind::Validation => {
        format!(
          "Validation failed for field `{}`: {}",
          self.field, self.message
        )
      }
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct PackageJsonParser {
  #[serde(flatten)]
  raw_fields: JsonMap<String, JsonValue>,

  #[serde(skip)]
  name: FieldResult<Name>,
  #[serde(skip)]
  version: FieldResult<Version>,
  #[serde(skip)]
  description: FieldResult<Description>,
  #[serde(skip)]
  keywords: FieldResult<Keywords>,
  #[serde(skip)]
  homepage: FieldResult<HomePage>,
  #[serde(skip)]
  bugs: FieldResult<Bugs>,
  #[serde(skip)]
  license: FieldResult<License>,
  #[serde(skip)]
  author: FieldResult<Person>,
  #[serde(skip)]
  contributors: FieldResult<Contributors>,
  #[serde(skip)]
  maintainers: FieldResult<Maintainers>,
  #[serde(skip)]
  files: FieldResult<Files>,
  #[serde(skip)]
  main: FieldResult<Main>,
  #[serde(skip)]
  r#type: FieldResult<Type>,
  #[serde(skip)]
  types: FieldResult<Types>,
  #[serde(skip)]
  typings: FieldResult<Typings>,
  #[serde(skip)]
  package_manager: FieldResult<PackageManager>,
  #[serde(skip)]
  publish_config: FieldResult<PublishConfig>,
  #[serde(skip)]
  bin: FieldResult<Bin>,
  #[serde(skip)]
  man: FieldResult<Man>,
  #[serde(skip)]
  directories: FieldResult<Directories>,
  #[serde(skip)]
  repository: FieldResult<RepositoryOrString>,
  #[serde(skip)]
  module: FieldResult<Module>,
  #[serde(skip)]
  readme: FieldResult<Readme>,
  #[serde(skip)]
  private: FieldResult<Private>,
  #[serde(skip)]
  engines: FieldResult<Engines>,
  #[serde(skip)]
  engine_strict: FieldResult<EngineStrict>,
  #[serde(skip)]
  os: FieldResult<Os>,
  #[serde(skip)]
  cpu: FieldResult<Cpu>,
  #[serde(skip)]
  scripts: FieldResult<Scripts>,
  #[serde(skip)]
  dependencies: FieldResult<Dependencies>,
  #[serde(skip)]
  dev_dependencies: FieldResult<DevDependencies>,
  #[serde(skip)]
  optional_dependencies: FieldResult<OptionalDependencies>,
  #[serde(skip)]
  peer_dependencies: FieldResult<PeerDependencies>,

  #[serde(skip)]
  __raw_source: Option<String>,
  #[serde(skip)]
  __raw_path: Option<String>,
}

impl<'de> Deserialize<'de> for PackageJsonParser {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let value = JsonValue::deserialize(deserializer)?;
    let Some(raw_fields) = value.as_object() else {
      return Err(serde::de::Error::custom(
        "package.json root must be a JSON object",
      ));
    };

    Ok(Self::new(raw_fields.clone(), None, None, None))
  }
}

macro_rules! define_getter {
  ($fn_name:ident, $field:ident, $ty:ty) => {
    pub fn $fn_name(&self) -> Result<Option<&$ty>> {
      self.field_result_to_ref(&self.$field)
    }
  };
}

impl PackageJsonParser {
  fn new(
    raw_fields: JsonMap<String, JsonValue>,
    raw_source: Option<String>,
    raw_path: Option<String>,
    root: Option<&jsonc_parser::ast::Object>,
  ) -> Self {
    let mut parser = Self {
      raw_fields,
      name: Ok(None),
      version: Ok(None),
      description: Ok(None),
      keywords: Ok(None),
      homepage: Ok(None),
      bugs: Ok(None),
      license: Ok(None),
      author: Ok(None),
      contributors: Ok(None),
      maintainers: Ok(None),
      files: Ok(None),
      main: Ok(None),
      r#type: Ok(None),
      types: Ok(None),
      typings: Ok(None),
      package_manager: Ok(None),
      publish_config: Ok(None),
      bin: Ok(None),
      man: Ok(None),
      directories: Ok(None),
      repository: Ok(None),
      module: Ok(None),
      readme: Ok(None),
      private: Ok(None),
      engines: Ok(None),
      engine_strict: Ok(None),
      os: Ok(None),
      cpu: Ok(None),
      scripts: Ok(None),
      dependencies: Ok(None),
      dev_dependencies: Ok(None),
      optional_dependencies: Ok(None),
      peer_dependencies: Ok(None),
      __raw_source: raw_source,
      __raw_path: raw_path,
    };

    parser.init_field_states(root);
    parser
  }

  fn init_field_states(&mut self, root: Option<&jsonc_parser::ast::Object>) {
    self.name = self.decode_field::<Name>("name", root.and_then(|obj| obj.get("name")));
    self.version = self.decode_field::<Version>("version", root.and_then(|obj| obj.get("version")));
    self.description =
      self.decode_field::<Description>("description", root.and_then(|obj| obj.get("description")));
    self.keywords =
      self.decode_field::<Keywords>("keywords", root.and_then(|obj| obj.get("keywords")));
    self.homepage =
      self.decode_field::<HomePage>("homepage", root.and_then(|obj| obj.get("homepage")));
    self.bugs = self.decode_field::<Bugs>("bugs", root.and_then(|obj| obj.get("bugs")));
    self.license = self.decode_field::<License>("license", root.and_then(|obj| obj.get("license")));
    self.author = self.decode_field::<Person>("author", root.and_then(|obj| obj.get("author")));
    self.contributors = self
      .decode_field::<Contributors>("contributors", root.and_then(|obj| obj.get("contributors")));
    self.maintainers =
      self.decode_field::<Maintainers>("maintainers", root.and_then(|obj| obj.get("maintainers")));
    self.files = self.decode_field::<Files>("files", root.and_then(|obj| obj.get("files")));
    self.main = self.decode_field::<Main>("main", root.and_then(|obj| obj.get("main")));
    self.r#type = self.decode_field::<Type>("type", root.and_then(|obj| obj.get("type")));
    self.types = self.decode_field::<Types>("types", root.and_then(|obj| obj.get("types")));
    self.typings = self.decode_field::<Typings>("typings", root.and_then(|obj| obj.get("typings")));
    self.package_manager = self.decode_field::<PackageManager>(
      "packageManager",
      root.and_then(|obj| obj.get("packageManager")),
    );
    self.publish_config = self.decode_field::<PublishConfig>(
      "publishConfig",
      root.and_then(|obj| obj.get("publishConfig")),
    );
    self.bin = self.decode_field::<Bin>("bin", root.and_then(|obj| obj.get("bin")));
    self.man = self.decode_field::<Man>("man", root.and_then(|obj| obj.get("man")));
    self.directories =
      self.decode_field::<Directories>("directories", root.and_then(|obj| obj.get("directories")));
    self.repository = self
      .decode_field::<RepositoryOrString>("repository", root.and_then(|obj| obj.get("repository")));
    self.module = self.decode_field::<Module>("module", root.and_then(|obj| obj.get("module")));
    self.readme = self.decode_field::<Readme>("readme", root.and_then(|obj| obj.get("readme")));
    self.private = self.decode_field::<Private>("private", root.and_then(|obj| obj.get("private")));
    self.engines = self.decode_field::<Engines>("engines", root.and_then(|obj| obj.get("engines")));
    self.engine_strict = self
      .decode_field::<EngineStrict>("engineStrict", root.and_then(|obj| obj.get("engineStrict")));
    self.os = self.decode_field::<Os>("os", root.and_then(|obj| obj.get("os")));
    self.cpu = self.decode_field::<Cpu>("cpu", root.and_then(|obj| obj.get("cpu")));
    self.scripts = self.decode_field::<Scripts>("scripts", root.and_then(|obj| obj.get("scripts")));
    self.dependencies = self
      .decode_field::<Dependencies>("dependencies", root.and_then(|obj| obj.get("dependencies")));
    self.dev_dependencies = self.decode_field::<DevDependencies>(
      "devDependencies",
      root.and_then(|obj| obj.get("devDependencies")),
    );
    self.optional_dependencies = self.decode_field::<OptionalDependencies>(
      "optionalDependencies",
      root.and_then(|obj| obj.get("optionalDependencies")),
    );
    self.peer_dependencies = self.decode_field::<PeerDependencies>(
      "peerDependencies",
      root.and_then(|obj| obj.get("peerDependencies")),
    );
  }

  fn decode_field<T>(&self, json_key: &'static str, prop: Option<&ObjectProp>) -> FieldResult<T>
  where
    T: DeserializeOwned + Validator,
  {
    let Some(raw_value) = self.raw_fields.get(json_key) else {
      return Ok(None);
    };

    let parsed = serde_json::from_value::<T>(raw_value.clone())
      .map_err(|error| FieldError::deserialize(json_key, error))?;

    parsed
      .validate(prop)
      .map_err(|error| FieldError::validation(json_key, error))?;

    Ok(Some(parsed))
  }

  fn field_error_to_report(&self, error: &FieldError) -> miette::Report {
    let diagnostic = MietteDiagnostic::new(error.message()).with_severity(Severity::Error);
    let report = miette::miette!(diagnostic);

    let Some(source) = self.__raw_source.as_ref() else {
      return report;
    };

    if let Some(path) = self.__raw_path.as_ref() {
      return report.with_source_code(NamedSource::new(path.clone(), source.clone()));
    }

    report.with_source_code(source.clone())
  }

  fn ensure_field_ok<T>(&self, state: &FieldResult<T>) -> miette::Result<()> {
    if let Err(error) = state {
      return Err(self.field_error_to_report(error));
    }
    Ok(())
  }

  fn field_result_to_ref<'a, T>(&'a self, state: &'a FieldResult<T>) -> Result<Option<&'a T>> {
    match state {
      Ok(Some(value)) => Ok(Some(value)),
      Ok(None) => Ok(None),
      Err(error) => Err(self.field_error_to_report(error)),
    }
  }

  pub fn validate(&self) -> miette::Result<()> {
    self.ensure_field_ok(&self.name)?;
    self.ensure_field_ok(&self.version)?;
    self.ensure_field_ok(&self.description)?;
    self.ensure_field_ok(&self.keywords)?;
    self.ensure_field_ok(&self.homepage)?;
    self.ensure_field_ok(&self.bugs)?;
    self.ensure_field_ok(&self.license)?;
    self.ensure_field_ok(&self.author)?;
    self.ensure_field_ok(&self.contributors)?;
    self.ensure_field_ok(&self.maintainers)?;
    self.ensure_field_ok(&self.files)?;
    self.ensure_field_ok(&self.main)?;
    self.ensure_field_ok(&self.r#type)?;
    self.ensure_field_ok(&self.types)?;
    self.ensure_field_ok(&self.typings)?;
    self.ensure_field_ok(&self.package_manager)?;
    self.ensure_field_ok(&self.publish_config)?;
    self.ensure_field_ok(&self.bin)?;
    self.ensure_field_ok(&self.man)?;
    self.ensure_field_ok(&self.directories)?;
    self.ensure_field_ok(&self.repository)?;
    self.ensure_field_ok(&self.module)?;
    self.ensure_field_ok(&self.readme)?;
    self.ensure_field_ok(&self.private)?;
    self.ensure_field_ok(&self.engines)?;
    self.ensure_field_ok(&self.engine_strict)?;
    self.ensure_field_ok(&self.os)?;
    self.ensure_field_ok(&self.cpu)?;
    self.ensure_field_ok(&self.scripts)?;
    self.ensure_field_ok(&self.dependencies)?;
    self.ensure_field_ok(&self.dev_dependencies)?;
    self.ensure_field_ok(&self.optional_dependencies)?;
    self.ensure_field_ok(&self.peer_dependencies)?;

    Ok(())
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

    miette::miette!(JsonParseError {
      src,
      primary_span: Some(SourceSpan::from(0..content.len())),
      other_spans: vec![LabeledSpan::new(Some("here".to_string()), offset, len)],
      source: Some(error),
      advice: Some("Please check the JSON syntax".to_string()),
    })
  }

  fn root_object_error(content: &str) -> miette::Report {
    let label = LabeledSpan::new(Some("here".to_string()), 0, content.len().max(1));
    let diagnostic = MietteDiagnostic::new("package.json root must be a JSON object")
      .with_label(label)
      .with_severity(Severity::Error);
    miette::miette!(diagnostic)
  }

  pub fn parse_str(content: &str) -> Result<Self> {
    let parsed_value: JsonValue = serde_json::from_str(content)
      .map_err(|error| Self::build_parse_error(content.to_string(), content, error))?;

    let Some(raw_fields) = parsed_value.as_object() else {
      return Err(Self::root_object_error(content));
    };

    let ast = parse_to_ast(
      content,
      &CollectOptions::default(),
      &ParseOptions::default(),
    )
    .ok();
    let root = ast
      .as_ref()
      .and_then(|result| result.value.as_ref())
      .and_then(|value| value.as_object());

    Ok(Self::new(
      raw_fields.clone(),
      Some(content.to_string()),
      None,
      root,
    ))
  }

  pub fn parse<P: AsRef<Path>>(path: P) -> Result<Self> {
    let file = File::open(path.as_ref()).map_err(ErrorKind::IoError)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader
      .read_to_string(&mut content)
      .map_err(ErrorKind::IoError)?;

    let parsed_value: JsonValue = serde_json::from_str(&content).map_err(|error| {
      let src = NamedSource::new(path.as_ref().to_string_lossy(), content.clone());
      Self::build_parse_error(src, &content, error)
    })?;

    let Some(raw_fields) = parsed_value.as_object() else {
      return Err(Self::root_object_error(&content));
    };

    let ast = parse_to_ast(
      &content,
      &CollectOptions::default(),
      &ParseOptions::default(),
    )
    .ok();
    let root = ast
      .as_ref()
      .and_then(|result| result.value.as_ref())
      .and_then(|value| value.as_object());

    Ok(Self::new(
      raw_fields.clone(),
      Some(content.clone()),
      Some(path.as_ref().to_string_lossy().to_string()),
      root,
    ))
  }

  define_getter!(name, name, Name);
  define_getter!(version, version, Version);
  define_getter!(description, description, Description);
  define_getter!(keywords, keywords, Keywords);
  define_getter!(homepage, homepage, HomePage);
  define_getter!(bugs, bugs, Bugs);
  define_getter!(license, license, License);
  define_getter!(author, author, Person);
  define_getter!(contributors, contributors, Contributors);
  define_getter!(maintainers, maintainers, Maintainers);
  define_getter!(files, files, Files);
  define_getter!(main, main, Main);
  define_getter!(r#type, r#type, Type);
  define_getter!(types, types, Types);
  define_getter!(typings, typings, Typings);
  define_getter!(package_manager, package_manager, PackageManager);
  define_getter!(publish_config, publish_config, PublishConfig);
  define_getter!(bin, bin, Bin);
  define_getter!(man, man, Man);
  define_getter!(directories, directories, Directories);
  define_getter!(repository, repository, RepositoryOrString);
  define_getter!(module, module, Module);
  define_getter!(readme, readme, Readme);
  define_getter!(private, private, Private);
  define_getter!(engines, engines, Engines);
  define_getter!(engine_strict, engine_strict, EngineStrict);
  define_getter!(os, os, Os);
  define_getter!(cpu, cpu, Cpu);
  define_getter!(scripts, scripts, Scripts);
  define_getter!(dependencies, dependencies, Dependencies);
  define_getter!(dev_dependencies, dev_dependencies, DevDependencies);
  define_getter!(
    optional_dependencies,
    optional_dependencies,
    OptionalDependencies
  );
  define_getter!(peer_dependencies, peer_dependencies, PeerDependencies);

  pub fn bin_to_hash_map(&self) -> Result<HashMap<String, String>> {
    let Some(bin) = self.bin()? else {
      return Ok(HashMap::default());
    };

    let bin = match bin {
      Bin::String(v) => {
        let mut map = HashMap::default();
        let Some(name) = self.name()? else {
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
  }

  #[test]
  fn should_defer_deserialize_error_until_field_access() {
    let parser = PackageJsonParser::parse_str(r#"{"packageManager":false}"#).unwrap();

    assert!(parser.package_manager().is_err());
    assert!(parser.validate().is_err());
  }

  #[test]
  fn should_report_getter_error_when_field_type_mismatches() {
    let parser = PackageJsonParser::parse_str(r#"{"name":"pkg","packageManager":false}"#).unwrap();

    assert!(parser.package_manager().is_err());
    assert!(parser.name().unwrap().is_some());
  }

  #[test]
  fn should_report_getter_error_when_field_validation_fails() {
    let parser =
      PackageJsonParser::parse_str(r#"{"name":"pkg","packageManager":"invalid"}"#).unwrap();

    assert!(parser.package_manager().is_err());
    assert!(parser.name().unwrap().is_some());
  }

  #[test]
  fn should_return_none_for_missing_field() {
    let parser = PackageJsonParser::parse_str(r#"{}"#).unwrap();
    let package_manager = parser.package_manager().unwrap();
    assert!(package_manager.is_none());
  }

  #[test]
  fn should_serialize_invalid_field_using_raw_value() {
    let parser = PackageJsonParser::parse_str(r#"{"packageManager":false,"name":"pkg"}"#).unwrap();
    let json_value = serde_json::to_value(parser).unwrap();

    assert_eq!(json_value["packageManager"], serde_json::json!(false));
    assert_eq!(json_value["name"], serde_json::json!("pkg"));
  }
}
