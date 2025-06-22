use package_json_parser::PackageJsonParser;

#[test]
fn should_pass_when_publish_config_is_valid() {
  let raw = r#"
        {
            "publishConfig": {
                "access": "public"
            }
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
fn should_fail_when_publish_config_is_invalid() {
  let raw = r#"
        {
            "publishConfig": "invalid"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    let res = package_json_parser.validate();
    assert!(res.is_err());
  }
}

#[test]
fn should_pass_when_publish_config_is_valid_with_registry() {
  let raw = r#"
        {
            "publishConfig": {
                "registry": "https://registry.npmjs.org"
            }
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
fn should_fail_when_publish_config_is_invalid_with_registry() {
  let raw = r#"
        {
            "publishConfig": {
                "registry": "invalid"
            }
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    let res = package_json_parser.validate();
    assert!(res.is_err());
  }
}

#[test]
fn should_pass_when_publish_config_is_valid_with_tag() {
  let raw = r#"
        {
            "publishConfig": {
                "tag": "latest"
            }
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
fn should_fail_when_publish_config_is_invalid_with_tag() {
  let raw = r#"
        {
            "publishConfig": {
                "tag": "alpha"
            }
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    let res = package_json_parser.validate();
    assert!(res.is_ok());
  }
}

#[test]
fn should_pass_when_publish_config_is_valid_with_provenance() {
  let raw = r#"
        {
            "publishConfig": {
                "provenance": true
            }
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());
}
