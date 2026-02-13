use package_json_parser::{PackageJsonParser, ValidationField};

fn invalid_payload() -> &'static str {
  r#"
  {
    "name": "MyPackage",
    "version": "invalid-version",
    "bugs": {
      "url": "invalid",
      "email": "invalid"
    }
  }
  "#
}

#[test]
fn validate_reports_errors_by_default() {
  let pkg = PackageJsonParser::parse_str(invalid_payload()).unwrap();
  let report = pkg.validate().unwrap();

  assert!(report.has_errors());
  assert!(report.warnings.is_empty());
}

#[test]
fn collect_all_issues_in_single_pass() {
  let pkg = PackageJsonParser::parse_str(invalid_payload()).unwrap();
  let report = pkg.validate().unwrap();

  assert!(report.errors.len() >= 4);
}

#[test]
fn nested_json_path_is_preserved() {
  let raw = r#"
  {
    "bugs": {
      "url": "https://example.com",
      "email": "invalid"
    }
  }
  "#;
  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  let report = pkg.validate().unwrap();

  assert!(
    report
      .errors
      .iter()
      .any(|issue| issue.field == ValidationField::Bugs && issue.json_path == "bugs.email")
  );
}

#[test]
fn missing_raw_source_is_fatal_error() {
  let mut pkg = PackageJsonParser::parse_str(r#"{ "name": "valid-name" }"#).unwrap();
  pkg.__raw_source = None;

  assert!(pkg.validate().is_err());
}
