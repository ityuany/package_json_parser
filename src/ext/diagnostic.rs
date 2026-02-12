use std::ops::Range;

use crate::validation::{RuleViolation, ValidationIssue};

/// 构建验证违规信息，由上层策略决定是 Error 还是 Warning。
pub fn validation_error(
  message: &str,
  code: Option<&str>,
  help: &str,
  range: Option<Range<usize>>,
  path: &'static str,
) -> RuleViolation {
  RuleViolation::new(message, code, Some(help), path, range)
}

/// 将校验结果转换为 miette 诊断，便于终端渲染。
#[allow(dead_code)]
pub fn issue_to_diagnostic(issue: &ValidationIssue) -> miette::MietteDiagnostic {
  issue.to_miette_diagnostic()
}
