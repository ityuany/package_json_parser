use package_json_parser::PackageJsonParser;

#[test]
fn should_pass_when_contributors_is_valid() {
  let raw = r#"
        {
            "contributors": ["test"]
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
fn should_pass_when_contributors_object_is_valid() {
  let raw = r#"
        {
            "contributors": [{ "name": "test" }]
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
fn should_fail_when_contributors_is_invalid() {
  let raw = r#"
        {
            "contributors": 123
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    assert!(package_json_parser.validate().is_err());
  }
}
