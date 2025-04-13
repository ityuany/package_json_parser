use package_json_parser::{PackageJsonParser, Validate};

#[test]
fn should_pass_when_main_is_valid() {
    let raw = r#"
        {
            "main": "test"
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(package_json_parser.main, Some("test".to_string()));

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_fail_when_main_is_invalid() {
    let raw = r#"
        {
            "main": 123
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    if let Ok(package_json_parser) = res {
        let res = package_json_parser.validate();
        assert!(res.is_err());
    }
}
