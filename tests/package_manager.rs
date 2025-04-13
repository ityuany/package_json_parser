use package_json_parser::{PackageJsonParser, PackageManager, Validate};

#[test]
fn should_pass_when_package_manager_is_valid() {
    let raw = r#"
        {
            "packageManager": "npm@1.0.0"
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(
            package_json_parser.package_manager,
            Some(PackageManager("npm@1.0.0".to_string()))
        );

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_fail_when_package_manager_is_invalid() {
    let raw = r#"
        {
            "packageManager": "invalid"
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    if let Ok(package_json_parser) = res {
        let res = package_json_parser.validate();
        assert!(res.is_err());
    }
}
