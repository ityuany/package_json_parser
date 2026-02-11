use std::ops::Range;

use miette::{LabeledSpan, MietteDiagnostic, Severity};

/// 构建验证错误的 miette::Report。
pub fn validation_error(
  message: &str,
  code: Option<&str>,
  help: &str,
  range: Option<Range<usize>>,
  label: &str,
) -> miette::Report {
  let mut diagnostic = MietteDiagnostic::new(message)
    .with_severity(Severity::Error)
    .with_help(help);

  if let Some(code) = code {
    diagnostic = diagnostic.with_code(code);
  }

  if let Some(range) = range {
    diagnostic = diagnostic.with_labels(vec![LabeledSpan::at(range, label)]);
  }

  miette::miette!(diagnostic)
}
