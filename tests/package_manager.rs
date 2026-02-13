use package_json_parser::PackageJsonParser;

#[test]
fn should_pass_when_package_manager_is_valid() {
  let raw = r#"
        {
            "packageManager": "npm@1.0.0"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let package_manager = package_json_parser.get_package_manager();
    assert_eq!(
      package_manager.value.as_ref().map(|v| v.as_str()),
      Some("npm@1.0.0")
    );
    assert!(!package_manager.has_errors());
  }
}

#[test]
fn should_fail_when_package_manager_is_invalid() {
  let raw = r#"
        {
            "packageManager": "invalid"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    let report = package_json_parser.validate().unwrap();
    assert!(report.has_errors());
  }
}
