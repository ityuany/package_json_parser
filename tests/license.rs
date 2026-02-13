use package_json_parser::PackageJsonParser;

#[test]
fn should_pass_when_license_is_valid() {
  let raw = r#"
        {
            "license": "MIT"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let license = package_json_parser.get_license();
    assert_eq!(license.value.as_ref().map(|v| v.as_str()), Some("MIT"));
    assert!(!license.has_errors());
  }
}

#[test]
fn should_fail_when_license_is_invalid() {
  let raw = r#"
        {
            "license": "MIT1"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    assert!(package_json_parser.validate().unwrap().has_errors());
  }
}
