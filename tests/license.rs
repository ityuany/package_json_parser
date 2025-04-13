use package_json_parser::{License, PackageJsonParser, Validate};

#[test]
fn should_pass_when_license_is_valid() {
    let raw = r#"
        {
            "license": "MIT"
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(
            package_json_parser.license,
            Some(License("MIT".to_string()))
        );

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_fail_when_license_is_invalid() {
    let raw = r#"
        {
            "license": "MIT1"
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    if let Ok(package_json_parser) = res {
        assert!(package_json_parser.validate().is_err());
    }
}
