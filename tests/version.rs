use package_json_parser::{PackageJsonParser, Validate, Version};

#[test]
fn should_pass_when_version_is_valid() {
  let raw = r#"
        {
            "version": "1.0.0"
        }
    "#;

  let res = serde_json::from_str::<PackageJsonParser>(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    assert_eq!(
      package_json_parser.version,
      Some(Version("1.0.0".to_string()))
    );

    let res = package_json_parser.validate();

    assert!(res.is_ok());
  }
}

#[test]
fn should_fail_when_version_is_invalid() {
  let raw = r#"
        {
            "version": "xxx"
        }
    "#;

  let res = serde_json::from_str::<PackageJsonParser>(raw);

  if let Ok(package_json_parser) = res {
    assert!(package_json_parser.validate().is_err());
  }
}
