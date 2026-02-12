use std::path::PathBuf;

use package_json_parser::{
  ErrorKind, PackageJsonParser, ValidationField, ValidationOptions, ValidationSeverity,
};

#[test]
fn parse_only_then_read_fields_without_validate() {
  let raw = r#"
  {
    "name": "demo-lib",
    "version": "1.2.3",
    "dependencies": { "serde": "^1.0.0" }
  }
  "#;

  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  assert_eq!(pkg.name.as_ref().map(|v| v.as_str()), Some("demo-lib"));
  assert_eq!(pkg.version.as_ref().map(|v| v.as_str()), Some("1.2.3"));
}

#[test]
fn lenient_and_strict_validation_have_different_behavior() {
  let raw = r#"
  {
    "name": "MyPackage",
    "version": "invalid-version",
    "bugs": "invalid"
  }
  "#;
  let pkg = PackageJsonParser::parse_str(raw).unwrap();

  let lenient = pkg.validate().unwrap();
  assert!(!lenient.has_errors());
  assert!(lenient.warnings.len() >= 3);

  let strict = pkg.validate_strict().unwrap();
  assert!(strict.has_errors());
  assert!(strict.errors.len() >= 3);
}

#[test]
fn global_and_field_level_policy_override_works() {
  let raw = r#"
  {
    "name": "MyPackage",
    "version": "invalid-version"
  }
  "#;
  let pkg = PackageJsonParser::parse_str(raw).unwrap();
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
fn strict_mode_can_downgrade_specific_field_to_warning() {
  let raw = r#"
  {
    "name": "MyPackage",
    "license": "MIT1"
  }
  "#;
  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  let options =
    ValidationOptions::error().with(ValidationField::License, ValidationSeverity::Warning);
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
      .any(|issue| issue.field == ValidationField::License)
  );
}

#[test]
fn nested_paths_are_visible_in_issues() {
  let raw = r#"
  {
    "bugs": { "url": "invalid", "email": "invalid" }
  }
  "#;
  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  let report = pkg.validate_strict().unwrap();

  assert!(
    report
      .errors
      .iter()
      .any(|issue| issue.json_path == "bugs.url")
  );
  assert!(
    report
      .errors
      .iter()
      .any(|issue| issue.json_path == "bugs.email")
  );
}

#[test]
fn parse_from_file_scenario() {
  let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("fixtures")
    .join("name-package.json");

  let pkg = PackageJsonParser::parse(path).unwrap();
  let report = pkg.validate().unwrap();

  assert!(!report.has_errors());
  assert!(!report.warnings.is_empty());
}

#[test]
fn invalid_json_fails_at_parse_stage() {
  let raw = r#"{ "name": "demo", }"#;
  assert!(PackageJsonParser::parse_str(raw).is_err());
}

#[test]
fn bin_string_with_scoped_name_maps_to_unscoped_bin_name() {
  let raw = r#"
  {
    "name": "@scope/tool",
    "bin": "./bin/tool.js"
  }
  "#;
  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  let bins = pkg.bin_to_hash_map().unwrap();

  assert_eq!(bins.get("tool"), Some(&"./bin/tool.js".to_string()));
}

#[test]
fn global_all_error_policy_marks_violations_as_errors() {
  let raw = r#"{ "name": "MyPackage", "version": "invalid-version" }"#;
  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  let report = pkg
    .validate_with(ValidationOptions::warning().all(ValidationSeverity::Error))
    .unwrap();

  assert!(report.has_errors());
  assert!(report.warnings.is_empty());
}

#[test]
fn same_field_override_last_write_wins() {
  let raw = r#"{ "name": "MyPackage" }"#;
  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  let options = ValidationOptions::error()
    .with(ValidationField::Name, ValidationSeverity::Error)
    .with(ValidationField::Name, ValidationSeverity::Warning);
  let report = pkg.validate_with(options).unwrap();

  assert!(!report.has_errors());
  assert_eq!(report.warnings.len(), 1);
  assert_eq!(report.warnings[0].field, ValidationField::Name);
}

#[test]
fn validation_report_helpers_are_consistent() {
  let raw = r#"{ "name": "MyPackage", "version": "invalid-version" }"#;
  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  let report = pkg.validate_strict().unwrap();

  assert!(report.has_errors());
  assert!(!report.is_clean());
  assert_eq!(
    report.issue_count(),
    report.errors.len() + report.warnings.len()
  );
}

#[test]
fn collect_all_keeps_multiple_issues_for_same_field() {
  let raw = r#"{ "bugs": { "url": "invalid", "email": "invalid" } }"#;
  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  let report = pkg.validate_strict().unwrap();

  let bugs_issues = report
    .errors
    .iter()
    .filter(|issue| issue.field == ValidationField::Bugs)
    .count();
  assert_eq!(bugs_issues, 2);
}

#[test]
fn top_level_issue_uses_top_level_json_path() {
  let raw = r#"{ "version": "invalid-version" }"#;
  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  let report = pkg.validate_strict().unwrap();

  assert!(
    report
      .errors
      .iter()
      .any(|issue| issue.json_path == "version")
  );
}

#[test]
fn try_from_str_and_path_are_available_for_users() {
  let raw = r#"{ "name": "demo-lib" }"#;
  let from_str = PackageJsonParser::try_from(raw).unwrap();
  assert_eq!(from_str.name.as_ref().map(|v| v.as_str()), Some("demo-lib"));

  let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("fixtures")
    .join("name-package.json");
  let from_path = PackageJsonParser::try_from(path.as_path()).unwrap();
  assert!(from_path.name.is_some());
}

#[test]
fn parsing_missing_file_returns_error() {
  let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("fixtures")
    .join("not-found-file.json");
  assert!(PackageJsonParser::parse(path).is_err());
}

#[test]
fn bin_to_hash_map_returns_empty_when_bin_absent() {
  let raw = r#"{ "name": "demo-lib" }"#;
  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  let bins = pkg.bin_to_hash_map().unwrap();

  assert!(bins.is_empty());
}

#[test]
fn bin_string_without_name_returns_name_required_error() {
  let raw = r#"{ "bin": "./cli.js" }"#;
  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  let err = pkg.bin_to_hash_map().unwrap_err();
  assert!(matches!(err.kind(), ErrorKind::NameRequired));
}
