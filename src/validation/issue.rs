use std::ops::Range;

use miette::{LabeledSpan, MietteDiagnostic, Severity};

use crate::validation::ValidationField;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationSeverity {
  Error,
  Warning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationIssueKind {
  TypeMismatch,
  SemanticViolation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleViolation {
  pub code: Option<String>,
  pub message: String,
  pub help: Option<String>,
  pub path: &'static str,
  pub span: Option<Range<usize>>,
}

impl RuleViolation {
  pub fn new(
    message: &str,
    code: Option<&str>,
    help: Option<&str>,
    path: &'static str,
    span: Option<Range<usize>>,
  ) -> Self {
    Self {
      code: code.map(ToString::to_string),
      message: message.to_string(),
      help: help.map(ToString::to_string),
      path,
      span,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
  pub field: ValidationField,
  pub kind: ValidationIssueKind,
  pub severity: ValidationSeverity,
  pub code: Option<String>,
  pub message: String,
  pub help: Option<String>,
  pub json_path: String,
  pub span: Option<Range<usize>>,
}

impl ValidationIssue {
  pub fn from_violation(
    field: ValidationField,
    severity: ValidationSeverity,
    violation: RuleViolation,
  ) -> Self {
    Self {
      field,
      kind: ValidationIssueKind::SemanticViolation,
      severity,
      code: violation.code,
      message: violation.message,
      help: violation.help,
      json_path: if violation.path.is_empty() {
        field.json_key().to_string()
      } else {
        format!("{}.{}", field.json_key(), violation.path)
      },
      span: violation.span,
    }
  }

  pub fn type_mismatch(
    field: ValidationField,
    severity: ValidationSeverity,
    message: String,
    help: Option<String>,
    span: Option<Range<usize>>,
  ) -> Self {
    Self {
      field,
      kind: ValidationIssueKind::TypeMismatch,
      severity,
      code: Some("type_mismatch".to_string()),
      message,
      help,
      json_path: field.json_key().to_string(),
      span,
    }
  }

  pub fn to_miette_diagnostic(&self) -> MietteDiagnostic {
    let severity = match self.severity {
      ValidationSeverity::Error => Severity::Error,
      ValidationSeverity::Warning => Severity::Warning,
    };

    let mut diagnostic = MietteDiagnostic::new(self.message.clone()).with_severity(severity);

    if let Some(code) = self.code.as_ref() {
      diagnostic = diagnostic.with_code(code.clone());
    }

    if let Some(help) = self.help.as_ref() {
      diagnostic = diagnostic.with_help(help.clone());
    }

    if let Some(span) = self.span.clone() {
      diagnostic = diagnostic.with_labels(vec![LabeledSpan::at(span, self.json_path.clone())]);
    }

    diagnostic
  }
}
