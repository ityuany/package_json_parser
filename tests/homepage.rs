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

#[test]
fn should_fail_parse_when_homepage_type_is_invalid() {
  let raw = [r#"{ "homepage": 123 }"#, r#"{ "homepage": true }"#];

  for raw in raw {
    let res = PackageJsonParser::parse_str(raw);
    assert!(res.is_err());
  }
}
