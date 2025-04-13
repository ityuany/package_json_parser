use package_json_parser::{PackageJsonParser, Validate};

#[test]
fn should_pass_when_homepage_is_valid() {
    let raw = r#"
        {
            "homepage": "https://test.com"
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(
            package_json_parser.homepage,
            Some("https://test.com".to_string())
        );

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}
