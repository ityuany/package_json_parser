use package_json_parser::PackageJsonParser;

#[test]
fn should_pass_when_homepage_is_valid() {
  let raw = r#"
        {
           "repository": {
              "type": "git"
            }
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let repository = package_json_parser.get_repository();
    assert!(repository.value.is_some());
    assert!(!repository.has_errors());
  }
}

#[test]
fn should_pass_when_repository_is_valid() {
  let raw = r#"
        {
           "repository": "https://github.com/npm/cli.git"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let repository = package_json_parser.get_repository();
    assert!(repository.value.is_some());
    assert!(!repository.has_errors());
  }
}
