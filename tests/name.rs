use package_json_parser::PackageJsonParser;

#[test]
fn should_pass_when_name_is_valid() {
  let raw = r#"
        {
            "name": "test"
        }
    "#;
  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let name = package_json_parser.get_name();
    assert_eq!(name.value.as_ref().map(|v| v.as_str()), Some("test"));
    assert!(!name.has_errors());
  }
}

#[test]
fn should_fail_when_name_is_invalid() {
  let raw = r#"
        {
            "name": "teYst"
        }
    "#;
  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    assert!(package_json_parser.validate().unwrap().has_errors());
  }
}
