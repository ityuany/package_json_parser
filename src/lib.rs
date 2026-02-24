pub use def::*;
use miette::{MietteDiagnostic, Severity};
use serde::de::{self, IgnoredAny, MapAccess, Visitor};

use crate::err::JsonParseError;
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

#[derive(Debug, Serialize, Clone)]
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

impl<'de> Deserialize<'de> for PackageJsonParser {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    enum Field {
      Name,
      Version,
      Description,
      Keywords,
      Homepage,
      Bugs,
      License,
      Author,
      Contributors,
      Maintainers,
      Files,
      Main,
      Type,
      Types,
      Typings,
      PackageManager,
      PublishConfig,
      Bin,
      Man,
      Directories,
      Repository,
      Module,
      Readme,
      Private,
      Engines,
      EngineStrict,
      Os,
      Cpu,
      Scripts,
      Dependencies,
      DevDependencies,
      OptionalDependencies,
      PeerDependencies,
      Ignore,
    }

    impl<'de> Deserialize<'de> for Field {
      fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
      where
        D: serde::Deserializer<'de>,
      {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
          type Value = Field;

          fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid package.json field")
          }

          fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
          where
            E: de::Error,
          {
            Ok(match value {
              "name" => Field::Name,
              "version" => Field::Version,
              "description" => Field::Description,
              "keywords" => Field::Keywords,
              "homepage" => Field::Homepage,
              "bugs" => Field::Bugs,
              "license" => Field::License,
              "author" => Field::Author,
              "contributors" => Field::Contributors,
              "maintainers" => Field::Maintainers,
              "files" => Field::Files,
              "main" => Field::Main,
              "type" => Field::Type,
              "types" => Field::Types,
              "typings" => Field::Typings,
              "packageManager" => Field::PackageManager,
              "publishConfig" => Field::PublishConfig,
              "bin" => Field::Bin,
              "man" => Field::Man,
              "directories" => Field::Directories,
              "repository" => Field::Repository,
              "module" => Field::Module,
              "readme" => Field::Readme,
              "private" => Field::Private,
              "engines" => Field::Engines,
              "engineStrict" => Field::EngineStrict,
              "os" => Field::Os,
              "cpu" => Field::Cpu,
              "scripts" => Field::Scripts,
              "dependencies" => Field::Dependencies,
              "devDependencies" => Field::DevDependencies,
              "optionalDependencies" => Field::OptionalDependencies,
              "peerDependencies" => Field::PeerDependencies,
              _ => Field::Ignore,
            })
          }
        }

        deserializer.deserialize_identifier(FieldVisitor)
      }
    }

    struct PackageJsonParserVisitor;

    impl<'de> Visitor<'de> for PackageJsonParserVisitor {
      type Value = PackageJsonParser;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a package.json object")
      }

      fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        let mut name = None;
        let mut version = None;
        let mut description = None;
        let mut keywords = None;
        let mut homepage = None;
        let mut bugs = None;
        let mut license = None;
        let mut author = None;
        let mut contributors = None;
        let mut maintainers = None;
        let mut files = None;
        let mut main = None;
        let mut r#type = None;
        let mut types = None;
        let mut typings = None;
        let mut package_manager = None;
        let mut publish_config = None;
        let mut bin = None;
        let mut man = None;
        let mut directories = None;
        let mut repository = None;
        let mut module = None;
        let mut readme = None;
        let mut private = None;
        let mut engines = None;
        let mut engine_strict = None;
        let mut os = None;
        let mut cpu = None;
        let mut scripts = None;
        let mut dependencies = None;
        let mut dev_dependencies = None;
        let mut optional_dependencies = None;
        let mut peer_dependencies = None;

        while let Some(field) = map.next_key()? {
          match field {
            Field::Name => {
              if name.is_some() {
                return Err(de::Error::duplicate_field("name"));
              }
              name = Some(map.next_value()?);
            }
            Field::Version => {
              if version.is_some() {
                return Err(de::Error::duplicate_field("version"));
              }
              version = Some(map.next_value()?);
            }
            Field::Description => {
              if description.is_some() {
                return Err(de::Error::duplicate_field("description"));
              }
              description = Some(map.next_value()?);
            }
            Field::Keywords => {
              if keywords.is_some() {
                return Err(de::Error::duplicate_field("keywords"));
              }
              keywords = Some(map.next_value()?);
            }
            Field::Homepage => {
              if homepage.is_some() {
                return Err(de::Error::duplicate_field("homepage"));
              }
              homepage = Some(map.next_value()?);
            }
            Field::Bugs => {
              if bugs.is_some() {
                return Err(de::Error::duplicate_field("bugs"));
              }
              bugs = Some(map.next_value()?);
            }
            Field::License => {
              if license.is_some() {
                return Err(de::Error::duplicate_field("license"));
              }
              license = Some(map.next_value()?);
            }
            Field::Author => {
              if author.is_some() {
                return Err(de::Error::duplicate_field("author"));
              }
              author = Some(map.next_value()?);
            }
            Field::Contributors => {
              if contributors.is_some() {
                return Err(de::Error::duplicate_field("contributors"));
              }
              contributors = Some(map.next_value()?);
            }
            Field::Maintainers => {
              if maintainers.is_some() {
                return Err(de::Error::duplicate_field("maintainers"));
              }
              maintainers = Some(map.next_value()?);
            }
            Field::Files => {
              if files.is_some() {
                return Err(de::Error::duplicate_field("files"));
              }
              files = Some(map.next_value()?);
            }
            Field::Main => {
              if main.is_some() {
                return Err(de::Error::duplicate_field("main"));
              }
              main = Some(map.next_value()?);
            }
            Field::Type => {
              if r#type.is_some() {
                return Err(de::Error::duplicate_field("type"));
              }
              r#type = Some(map.next_value()?);
            }
            Field::Types => {
              if types.is_some() {
                return Err(de::Error::duplicate_field("types"));
              }
              types = Some(map.next_value()?);
            }
            Field::Typings => {
              if typings.is_some() {
                return Err(de::Error::duplicate_field("typings"));
              }
              typings = Some(map.next_value()?);
            }
            Field::PackageManager => {
              if package_manager.is_some() {
                return Err(de::Error::duplicate_field("packageManager"));
              }
              package_manager = Some(map.next_value()?);
            }
            Field::PublishConfig => {
              if publish_config.is_some() {
                return Err(de::Error::duplicate_field("publishConfig"));
              }
              publish_config = Some(map.next_value()?);
            }
            Field::Bin => {
              if bin.is_some() {
                return Err(de::Error::duplicate_field("bin"));
              }
              bin = Some(map.next_value()?);
            }
            Field::Man => {
              if man.is_some() {
                return Err(de::Error::duplicate_field("man"));
              }
              man = Some(map.next_value()?);
            }
            Field::Directories => {
              if directories.is_some() {
                return Err(de::Error::duplicate_field("directories"));
              }
              directories = Some(map.next_value()?);
            }
            Field::Repository => {
              if repository.is_some() {
                return Err(de::Error::duplicate_field("repository"));
              }
              repository = Some(map.next_value()?);
            }
            Field::Module => {
              if module.is_some() {
                return Err(de::Error::duplicate_field("module"));
              }
              module = Some(map.next_value()?);
            }
            Field::Readme => {
              if readme.is_some() {
                return Err(de::Error::duplicate_field("readme"));
              }
              readme = Some(map.next_value()?);
            }
            Field::Private => {
              if private.is_some() {
                return Err(de::Error::duplicate_field("private"));
              }
              private = Some(map.next_value()?);
            }
            Field::Engines => {
              if engines.is_some() {
                return Err(de::Error::duplicate_field("engines"));
              }
              engines = Some(map.next_value()?);
            }
            Field::EngineStrict => {
              if engine_strict.is_some() {
                return Err(de::Error::duplicate_field("engineStrict"));
              }
              engine_strict = Some(map.next_value()?);
            }
            Field::Os => {
              if os.is_some() {
                return Err(de::Error::duplicate_field("os"));
              }
              os = Some(map.next_value()?);
            }
            Field::Cpu => {
              if cpu.is_some() {
                return Err(de::Error::duplicate_field("cpu"));
              }
              cpu = Some(map.next_value()?);
            }
            Field::Scripts => {
              if scripts.is_some() {
                return Err(de::Error::duplicate_field("scripts"));
              }
              scripts = Some(map.next_value()?);
            }
            Field::Dependencies => {
              if dependencies.is_some() {
                return Err(de::Error::duplicate_field("dependencies"));
              }
              dependencies = Some(map.next_value()?);
            }
            Field::DevDependencies => {
              if dev_dependencies.is_some() {
                return Err(de::Error::duplicate_field("devDependencies"));
              }
              dev_dependencies = Some(map.next_value()?);
            }
            Field::OptionalDependencies => {
              if optional_dependencies.is_some() {
                return Err(de::Error::duplicate_field("optionalDependencies"));
              }
              optional_dependencies = Some(map.next_value()?);
            }
            Field::PeerDependencies => {
              if peer_dependencies.is_some() {
                return Err(de::Error::duplicate_field("peerDependencies"));
              }
              peer_dependencies = Some(map.next_value()?);
            }
            Field::Ignore => {
              let _ = map.next_value::<IgnoredAny>()?;
            }
          }
        }

        Ok(PackageJsonParser {
          name,
          version,
          description,
          keywords,
          homepage,
          bugs,
          license,
          author,
          contributors,
          maintainers,
          files,
          main,
          r#type,
          types,
          typings,
          package_manager,
          publish_config,
          bin,
          man,
          directories,
          repository,
          module,
          readme,
          private,
          engines,
          engine_strict,
          os,
          cpu,
          scripts,
          dependencies,
          dev_dependencies,
          optional_dependencies,
          peer_dependencies,
          __raw_source: None,
          __raw_path: None,
        })
      }
    }

    deserializer.deserialize_map(PackageJsonParserVisitor)
  }
}

impl PackageJsonParser {
  fn raw_source(&self) -> miette::Result<&str> {
    self
      .__raw_source
      .as_deref()
      .ok_or_else(|| miette::miette!("raw source is not available, was this parsed correctly?"))
  }

  fn handle_error(&self, e: miette::Result<()>) -> miette::Result<()> {
    let Err(e) = e else {
      return Ok(());
    };
    let source = self.raw_source()?.to_string();
    if let Some(path) = self.__raw_path.as_ref() {
      return Err(e.with_source_code(NamedSource::new(path, source)));
    }
    Err(e.with_source_code(source))
  }

  pub fn validate(&self) -> miette::Result<()> {
    let Ok(parse_result) = parse_to_ast(
      self.raw_source()?,
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

  pub fn parse_str(content: &str) -> Result<Self> {
    let mut package_json_parser: PackageJsonParser = serde_json::from_str(content)
      .map_err(|e| Self::build_parse_error(content.to_string(), content, e))?;
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
        let src = NamedSource::new(path.as_ref().to_string_lossy(), content.clone());
        Self::build_parse_error(src, &content, e)
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
