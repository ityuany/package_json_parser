use package_json_parser::PackageJsonParser;

#[test]
fn should_pass_when_version_is_valid() {
  let raw = r#"
        {
            "version": "1.0.0"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let version = package_json_parser.get_version();
    assert_eq!(version.value.as_ref().map(|v| v.as_str()), Some("1.0.0"));
    assert!(!version.has_errors());
  }
}

#[test]
fn should_fail_when_version_is_invalid() {
  let raw = r#"
        {
            "version": "xxx"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    assert!(package_json_parser.validate().unwrap().has_errors());
  }
}
