use miette::NamedSource;
use serde::de::DeserializeOwned;

use crate::ext::Validator;
use crate::{
  Bin, Bugs, Contributors, Cpu, Dependencies, Description, DevDependencies, Directories,
  EngineStrict, Engines, Files, HomePage, Keywords, License, Main, Maintainers, Man, Module, Name,
  OptionalDependencies, Os, PackageJsonError, PackageJsonParser, PackageManager, PeerDependencies,
  Person, Private, PublishConfig, Readme, RepositoryOrString, Result, Scripts, Type, Types,
  Typings, ValidationField, ValidationIssue, ValidationReport, ValidationSeverity, Version,
};
use jsonc_parser::{CollectOptions, ParseOptions, parse_to_ast};

macro_rules! getter {
  ($name:ident, $ty:ty, $field:expr) => {
    pub fn $name(&self) -> FieldResult<$ty> {
      self.decode_field::<$ty>($field)
    }
  };
}

#[derive(Debug, Clone, Default)]
pub struct FieldResult<T> {
  pub value: Option<T>,
  pub issues: Vec<ValidationIssue>,
}

impl<T> FieldResult<T> {
  pub fn has_errors(&self) -> bool {
    self
      .issues
      .iter()
      .any(|issue| issue.severity == ValidationSeverity::Error)
  }

  pub fn has_warnings(&self) -> bool {
    self
      .issues
      .iter()
      .any(|issue| issue.severity == ValidationSeverity::Warning)
  }
}

#[derive(Debug, Clone)]
pub struct RenderContext {
  source: String,
  path: Option<String>,
}

impl RenderContext {
  pub fn render_issue(&self, issue: &ValidationIssue) -> String {
    let diagnostic = issue.to_miette_diagnostic();
    let report = if let Some(path) = self.path.as_ref() {
      miette::Report::new(diagnostic).with_source_code(NamedSource::new(path, self.source.clone()))
    } else {
      miette::Report::new(diagnostic).with_source_code(self.source.clone())
    };
    format!("{report:?}")
  }

  pub fn render_report(&self, report: &ValidationReport) -> String {
    report
      .errors
      .iter()
      .chain(report.warnings.iter())
      .map(|issue| self.render_issue(issue))
      .collect::<Vec<_>>()
      .join("\n\n")
  }
}

impl PackageJsonParser {
  fn decode_field<T: DeserializeOwned + Validator>(
    &self,
    field: ValidationField,
  ) -> FieldResult<T> {
    let value = self
      .raw_json()
      .ok()
      .and_then(|json| json.get(field.json_key()))
      .and_then(|raw| serde_json::from_value::<T>(raw.clone()).ok());

    let issues = match parse_to_ast(
      self.raw_source().unwrap_or("{}"),
      &CollectOptions::default(),
      &ParseOptions::default(),
    ) {
      Ok(parse_result) => {
        let root = parse_result
          .value
          .as_ref()
          .and_then(|value| value.as_object());
        self
          .collect_field_issues::<T>(field, root)
          .unwrap_or_default()
      }
      Err(_) => self
        .collect_field_issues::<T>(field, None)
        .unwrap_or_default(),
    };

    FieldResult { value, issues }
  }

  pub fn render_context(&self) -> Result<RenderContext> {
    let source = self.__raw_source.clone().ok_or_else(|| {
      PackageJsonError::InternalState(
        "raw source is not available, was this parsed correctly?".to_string(),
      )
    })?;

    Ok(RenderContext {
      source,
      path: self.__raw_path.clone(),
    })
  }

  pub fn render_issue(&self, issue: &ValidationIssue) -> String {
    match self.render_context() {
      Ok(ctx) => ctx.render_issue(issue),
      Err(_) => issue.message.clone(),
    }
  }

  pub fn render_report(&self, report: &ValidationReport) -> String {
    match self.render_context() {
      Ok(ctx) => ctx.render_report(report),
      Err(_) => report
        .errors
        .iter()
        .chain(report.warnings.iter())
        .map(|issue| issue.message.clone())
        .collect::<Vec<_>>()
        .join("\n"),
    }
  }

  getter!(get_name, Name, ValidationField::Name);
  getter!(get_version, Version, ValidationField::Version);
  getter!(get_description, Description, ValidationField::Description);
  getter!(get_keywords, Keywords, ValidationField::Keywords);
  getter!(get_homepage, HomePage, ValidationField::Homepage);
  getter!(get_bugs, Bugs, ValidationField::Bugs);
  getter!(get_license, License, ValidationField::License);
  getter!(get_author, Person, ValidationField::Author);
  getter!(
    get_contributors,
    Contributors,
    ValidationField::Contributors
  );
  getter!(get_maintainers, Maintainers, ValidationField::Maintainers);
  getter!(get_files, Files, ValidationField::Files);
  getter!(get_main, Main, ValidationField::Main);
  getter!(get_type, Type, ValidationField::Type);
  getter!(get_types, Types, ValidationField::Types);
  getter!(get_typings, Typings, ValidationField::Typings);
  getter!(
    get_package_manager,
    PackageManager,
    ValidationField::PackageManager
  );
  getter!(
    get_publish_config,
    PublishConfig,
    ValidationField::PublishConfig
  );
  getter!(get_bin, Bin, ValidationField::Bin);
  getter!(get_man, Man, ValidationField::Man);
  getter!(get_directories, Directories, ValidationField::Directories);
  getter!(
    get_repository,
    RepositoryOrString,
    ValidationField::Repository
  );
  getter!(get_module, Module, ValidationField::Module);
  getter!(get_readme, Readme, ValidationField::Readme);
  getter!(get_private, Private, ValidationField::Private);
  getter!(get_engines, Engines, ValidationField::Engines);
  getter!(
    get_engine_strict,
    EngineStrict,
    ValidationField::EngineStrict
  );
  getter!(get_os, Os, ValidationField::Os);
  getter!(get_cpu, Cpu, ValidationField::Cpu);
  getter!(get_scripts, Scripts, ValidationField::Scripts);
  getter!(
    get_dependencies,
    Dependencies,
    ValidationField::Dependencies
  );
  getter!(
    get_dev_dependencies,
    DevDependencies,
    ValidationField::DevDependencies
  );
  getter!(
    get_optional_dependencies,
    OptionalDependencies,
    ValidationField::OptionalDependencies
  );
  getter!(
    get_peer_dependencies,
    PeerDependencies,
    ValidationField::PeerDependencies
  );
}
