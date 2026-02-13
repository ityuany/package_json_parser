use package_json_parser::PackageJsonParser;

#[test]
fn should_pass_when_description_is_valid() {
  let raw = r#"
        {
            "description": "test"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let description = package_json_parser.get_description();
    assert_eq!(description.value.as_ref().map(|v| v.as_str()), Some("test"));
    assert!(!description.has_errors());
  }
}

#[test]
fn should_fail_when_description_is_invalid() {
  let raw = r#"
        {
            "description": 123
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    assert!(package_json_parser.validate().unwrap().has_errors());
  }
}
