use package_json_parser::PackageJsonParser;

#[test]
fn should_pass_when_files_is_valid() {
  let raw = r#"
        {
            "files": ["test"]
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let files = package_json_parser.get_files();
    assert_eq!(files.value.as_ref().map(|v| v.len()), Some(1));
    assert!(!files.has_errors());
  }
}

#[test]
fn should_fail_when_files_is_invalid() {
  let raw = r#"
        {
            "files": 123
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    assert!(package_json_parser.validate().unwrap().has_errors());
  }
}
