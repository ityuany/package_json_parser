use crate::validation::{ValidationIssue, ValidationSeverity};

#[derive(Debug, Clone, Default)]
pub struct ValidationReport {
  pub errors: Vec<ValidationIssue>,
  pub warnings: Vec<ValidationIssue>,
}

impl ValidationReport {
  pub fn push(&mut self, issue: ValidationIssue) {
    match issue.severity {
      ValidationSeverity::Error => self.errors.push(issue),
      ValidationSeverity::Warning => self.warnings.push(issue),
    }
  }

  pub fn push_many<I>(&mut self, issues: I)
  where
    I: IntoIterator<Item = ValidationIssue>,
  {
    for issue in issues {
      self.push(issue);
    }
  }

  pub fn has_errors(&self) -> bool {
    !self.errors.is_empty()
  }

  pub fn is_clean(&self) -> bool {
    self.errors.is_empty() && self.warnings.is_empty()
  }

  pub fn issue_count(&self) -> usize {
    self.errors.len() + self.warnings.len()
  }
}
