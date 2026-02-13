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
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let publish_config = package_json_parser.get_publish_config();
    assert!(publish_config.value.is_some());
    assert!(!publish_config.has_errors());
  }
}

#[test]
fn should_fail_when_publish_config_is_invalid() {
  let raw = r#"
        {
            "publishConfig": "invalid"
        }
    "#;

  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  let report = pkg.validate().unwrap();
  assert!(report.has_errors());
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
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let publish_config = package_json_parser.get_publish_config();
    assert!(publish_config.value.is_some());
    assert!(!publish_config.has_errors());
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
    let report = package_json_parser.validate().unwrap();
    assert!(report.has_errors());
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
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let publish_config = package_json_parser.get_publish_config();
    assert!(publish_config.value.is_some());
    assert!(!publish_config.has_errors());
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
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let publish_config = package_json_parser.get_publish_config();
    assert!(publish_config.value.is_some());
    assert!(!publish_config.has_errors());
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
  let package_json_parser = res.unwrap();
  let report = package_json_parser.validate().unwrap();
  assert!(!report.has_errors());
  let publish_config = package_json_parser.get_publish_config();
  assert!(publish_config.value.is_some());
  assert!(!publish_config.has_errors());
}
