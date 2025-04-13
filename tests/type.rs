use package_json_parser::{PackageJsonParser, Type, Validate};

#[test]
fn should_pass_when_type_is_valid() {
    let raw = r#"
        {
            "type": "module"
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(package_json_parser.r#type, Some(Type("module".to_string())));

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_fail_when_type_is_invalid() {
    let raw = r#"
        {
            "type": "invalid"
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    if let Ok(package_json_parser) = res {
        assert!(package_json_parser.validate().is_err());
    }
}
