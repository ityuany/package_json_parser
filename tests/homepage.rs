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
    let res = package_json_parser.validate();

    assert!(res.is_ok());
  }
}
