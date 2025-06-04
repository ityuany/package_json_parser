use miette::{Diagnostic, LabeledSpan, NamedSource, SourceSpan};

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum ErrorKind {
  #[error("name is required")]
  NameRequired,

  #[error("JSON parsing failed")]
  #[diagnostic(code(package_json::json_parse_failed), url(docsrs))]
  JsonParseError {
    #[source_code]
    src: NamedSource<String>,

    #[label]
    primary_span: Option<SourceSpan>,

    #[label(collection, "related to this")]
    other_spans: Vec<LabeledSpan>,

    #[help]
    advice: Option<String>,

    #[source]
    source: Option<serde_json::Error>,
  },

  #[error("IO error")]
  #[diagnostic(code(package_json::io_error), url(docsrs))]
  IoError(#[from] std::io::Error),
}
