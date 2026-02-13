use package_json_parser::{Bugs, PackageJsonParser};

#[test]
fn should_pass_when_bugs_is_valid_url() {
  let raw = r#"
        {
            "bugs": "https://test.com"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    assert_eq!(
      package_json_parser.bugs,
      Some(Bugs::UrlOrEmail("https://test.com".to_string()))
    );

    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let bugs = package_json_parser.get_bugs();
    assert!(bugs.value.is_some());
    assert!(!bugs.has_errors());
  }
}

#[test]
fn should_pass_when_bugs_is_valid_email() {
  let raw = r#"
        {
            "bugs": "test@example.com"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    assert_eq!(
      package_json_parser.bugs,
      Some(Bugs::UrlOrEmail("test@example.com".to_string()))
    );

    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let bugs = package_json_parser.get_bugs();
    assert!(bugs.value.is_some());
    assert!(!bugs.has_errors());
  }
}

#[test]
fn should_pass_when_bugs_is_valid_object() {
  let raw = r#"
    {
        "bugs": {
            "url": "https://test.com",
            "email": "test@example.com"
        }
    }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
    let bugs = package_json_parser.get_bugs();
    assert!(bugs.value.is_some());
    assert!(!bugs.has_errors());
  }
}

#[test]
fn should_fail_when_bugs_is_invalid() {
  let raw = r#"
        {
            "bugs": "invalid"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    assert!(package_json_parser.validate().unwrap().has_errors());
  }
}

#[test]
fn should_fail_when_bugs_object_is_invalid() {
  let raw = r#"
        {
            "bugs": {
                "url": "https://test.com",
                "email": "xxx"
            }
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    let report = package_json_parser.validate().unwrap();
    assert!(report.has_errors());
  }
}
