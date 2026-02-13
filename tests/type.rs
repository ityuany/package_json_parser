use package_json_parser::PackageJsonParser;

#[test]
fn should_pass_when_type_is_valid() {
  let raw = r#"
        {
            "type": "module"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let r#type = package_json_parser.get_type();
    assert_eq!(r#type.value.as_ref().map(|v| v.as_str()), Some("module"));
    assert!(!r#type.has_errors());
  }
}

#[test]
fn should_fail_when_type_is_invalid() {
  let raw = r#"
        {
            "type": "invalid"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    assert!(package_json_parser.validate().unwrap().has_errors());
  }
}
