#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
  #[error("name is required")]
  NameRequired,

  #[error("JSON parsing failed")]
  JsonParseError {
    #[source]
    source: Option<serde_json::Error>,
  },

  #[error("IO error")]
  IoError(#[from] std::io::Error),
}
