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

pub type JsonFileParseError = JsonParseError<NamedSource<String>>;
pub type JsonStrParseError = JsonParseError<String>;

#[derive(Debug, Error, Diagnostic)]
pub enum ErrorKind {
  #[error("name is required")]
  NameRequired,

  #[error(transparent)]
  JsonFileParseError(JsonFileParseError),

  #[error(transparent)]
  JsonStrParseError(JsonStrParseError),

  #[error("IO error")]
  #[diagnostic(code(package_json::io_error), url(docsrs))]
  IoError(#[from] std::io::Error),
}
