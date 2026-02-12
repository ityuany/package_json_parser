use std::fmt::Debug;

use miette::{Diagnostic, LabeledSpan, NamedSource, SourceCode, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("JSON parsing failed")]
#[diagnostic(code(package_json::json_parse_failed), url(docsrs))]
pub struct JsonParseError<S>
where
  S: SourceCode + Debug,
{
  #[source_code]
  pub src: S,

  #[label]
  pub primary_span: Option<SourceSpan>,

  #[label(collection, "related to this")]
  pub other_spans: Vec<LabeledSpan>,

  #[help]
  pub advice: Option<String>,

  #[source]
  pub source: Option<serde_json::Error>,
}

#[allow(dead_code)]
pub type JsonFileParseError = JsonParseError<NamedSource<String>>;
#[allow(dead_code)]
pub type JsonStrParseError = JsonParseError<String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
  NameRequired,
  IoError,
  JsonParse,
  Validation,
  InternalState,
}

#[derive(Debug, Error)]
pub enum PackageJsonError {
  #[error("name is required")]
  NameRequired,

  #[error("I/O error")]
  Io(#[source] std::io::Error),

  #[error("JSON parsing failed")]
  JsonParse(miette::Report),

  #[error("validation failed")]
  Validation(miette::Report),

  #[error("internal state error: {0}")]
  InternalState(String),
}

impl PackageJsonError {
  pub fn kind(&self) -> ErrorKind {
    match self {
      PackageJsonError::NameRequired => ErrorKind::NameRequired,
      PackageJsonError::Io(_) => ErrorKind::IoError,
      PackageJsonError::JsonParse(_) => ErrorKind::JsonParse,
      PackageJsonError::Validation(_) => ErrorKind::Validation,
      PackageJsonError::InternalState(_) => ErrorKind::InternalState,
    }
  }
}

impl Diagnostic for PackageJsonError {
  fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
    match self {
      PackageJsonError::JsonParse(report) => Some(report.as_ref()),
      PackageJsonError::Validation(report) => Some(report.as_ref()),
      _ => None,
    }
  }
}

impl From<std::io::Error> for PackageJsonError {
  fn from(value: std::io::Error) -> Self {
    Self::Io(value)
  }
}

pub type Result<T> = std::result::Result<T, PackageJsonError>;
