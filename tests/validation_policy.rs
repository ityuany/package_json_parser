use package_json_parser::{
  PackageJsonParser, ValidationField, ValidationOptions, ValidationSeverity,
};

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
fn validate_is_lenient_by_default() {
  let pkg = PackageJsonParser::parse_str(invalid_payload()).unwrap();
  let report = pkg.validate().unwrap();

  assert!(!report.is_clean());
  assert!(!report.has_errors());
  assert!(report.warnings.len() >= 3);
}

#[test]
fn validate_with_error_reports_errors() {
  let pkg = PackageJsonParser::parse_str(invalid_payload()).unwrap();
  let report = pkg.validate_with(package_json_parser::ValidationOptions::error()).unwrap();

  assert!(report.has_errors());
  assert!(report.errors.len() >= 3);
}

#[test]
fn field_override_takes_precedence_over_global_default() {
  let pkg = PackageJsonParser::parse_str(invalid_payload()).unwrap();
  let options =
    ValidationOptions::warning().with(ValidationField::Name, ValidationSeverity::Error);
  let report = pkg.validate_with(options).unwrap();

  assert!(
    report
      .errors
      .iter()
      .any(|issue| issue.field == ValidationField::Name)
  );
  assert!(
    report
      .warnings
      .iter()
      .any(|issue| issue.field == ValidationField::Version)
  );
}

#[test]
fn error_mode_can_downgrade_specific_field_to_warning() {
  let raw = r#"{ "license": "MIT1" }"#;
  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  let options =
    ValidationOptions::error().with(ValidationField::License, ValidationSeverity::Warning);
  let report = pkg.validate_with(options).unwrap();

  assert!(!report.has_errors());
  assert_eq!(report.warnings.len(), 1);
  assert_eq!(report.warnings[0].field, ValidationField::License);
}

#[test]
fn collect_all_issues_in_single_pass() {
  let pkg = PackageJsonParser::parse_str(invalid_payload()).unwrap();
  let report = pkg.validate_with(package_json_parser::ValidationOptions::error()).unwrap();

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
  let report = pkg.validate_with(package_json_parser::ValidationOptions::error()).unwrap();

  assert!(
    report
      .errors
      .iter()
      .any(|issue| issue.json_path == "bugs.email")
  );
}

#[test]
fn missing_raw_source_is_fatal_error() {
  let mut pkg = PackageJsonParser::parse_str(r#"{ "name": "valid-name" }"#).unwrap();
  pkg.__raw_source = None;

  assert!(pkg.validate().is_err());
}
