#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
  #[error("name is required")]
  NameRequired,
}
