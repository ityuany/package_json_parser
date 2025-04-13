use package_json_parser::{Bugs, PackageJsonParser, Validate};

#[test]
fn should_pass_when_bugs_is_valid_url() {
  let raw = r#"
        {
            "bugs": "https://test.com"
        }
    "#;

  let res = serde_json::from_str::<PackageJsonParser>(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    assert_eq!(
      package_json_parser.bugs,
      Some(Bugs::UrlOrEmail("https://test.com".to_string()))
    );

    let res = package_json_parser.validate();

    assert!(res.is_ok());
  }
}

#[test]
fn should_pass_when_bugs_is_valid_email() {
  let raw = r#"
        {
            "bugs": "test@example.com"
        }
    "#;

  let res = serde_json::from_str::<PackageJsonParser>(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    assert_eq!(
      package_json_parser.bugs,
      Some(Bugs::UrlOrEmail("test@example.com".to_string()))
    );

    let res = package_json_parser.validate();

    assert!(res.is_ok());
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

  let res = serde_json::from_str::<PackageJsonParser>(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    let res = package_json_parser.validate();
    assert!(res.is_ok());
  }
}

#[test]
fn should_fail_when_bugs_is_invalid() {
  let raw = r#"
        {
            "bugs": "invalid"
        }
    "#;

  let res = serde_json::from_str::<PackageJsonParser>(raw);

  if let Ok(package_json_parser) = res {
    assert!(package_json_parser.validate().is_err());
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

  let res = serde_json::from_str::<PackageJsonParser>(raw);

  if let Ok(package_json_parser) = res {
    let res = package_json_parser.validate();
    assert!(res.is_err());
  }
}
