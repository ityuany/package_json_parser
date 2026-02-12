use std::collections::HashMap;

use crate::validation::{ValidationField, ValidationSeverity};

#[derive(Debug, Clone)]
pub struct ValidationOptions {
  default_severity: ValidationSeverity,
  field_overrides: HashMap<ValidationField, ValidationSeverity>,
}

impl Default for ValidationOptions {
  fn default() -> Self {
    Self::lenient()
  }
}

impl ValidationOptions {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn lenient() -> Self {
    Self {
      default_severity: ValidationSeverity::Warning,
      field_overrides: HashMap::new(),
    }
  }

  pub fn strict() -> Self {
    Self {
      default_severity: ValidationSeverity::Error,
      field_overrides: HashMap::new(),
    }
  }

  pub fn all(mut self, sev: ValidationSeverity) -> Self {
    self.default_severity = sev;
    self
  }

  pub fn field(mut self, field: ValidationField, sev: ValidationSeverity) -> Self {
    self.field_overrides.insert(field, sev);
    self
  }

  pub fn severity_for(&self, field: ValidationField) -> ValidationSeverity {
    self
      .field_overrides
      .get(&field)
      .copied()
      .unwrap_or(self.default_severity)
  }
}
