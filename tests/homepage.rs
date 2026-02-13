use package_json_parser::PackageJsonParser;

#[test]
fn should_pass_when_homepage_is_valid() {
  let raw = r#"
        {
            "homepage": "https://test.com"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let homepage = package_json_parser.get_homepage();
    assert_eq!(
      homepage.value.as_ref().map(|v| v.as_str()),
      Some("https://test.com")
    );
    assert!(!homepage.has_errors());
  }
}
