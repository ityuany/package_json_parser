use package_json_parser::{PackageJsonParser, Validate};

#[test]
fn should_pass_when_files_is_valid() {
  let raw = r#"
        {
            "files": ["test"]
        }
    "#;

  let res = serde_json::from_str::<PackageJsonParser>(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    assert_eq!(package_json_parser.files, Some(vec!["test".to_string()]));

    let res = package_json_parser.validate();

    assert!(res.is_ok());
  }
}

#[test]
fn should_fail_when_files_is_invalid() {
  let raw = r#"
        {
            "files": 123
        }
    "#;

  let res = serde_json::from_str::<PackageJsonParser>(raw);

  if let Ok(package_json_parser) = res {
    assert!(package_json_parser.validate().is_err());
  }
}
