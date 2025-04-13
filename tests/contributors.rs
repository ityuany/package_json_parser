use package_json_parser::{PackageJsonParser, Person, PersonObject, Validate};

#[test]
fn should_pass_when_contributors_is_valid() {
    let raw = r#"
        {
            "contributors": ["test"]
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(
            package_json_parser.contributors,
            Some(vec![Person::String("test".to_string())])
        );

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_pass_when_contributors_object_is_valid() {
    let raw = r#"
        {
            "contributors": [{ "name": "test" }]
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(
            package_json_parser.contributors,
            Some(vec![Person::Object(PersonObject {
                name: "test".to_string(),
                email: None,
                url: None
            })])
        );

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_fail_when_contributors_is_invalid() {
    let raw = r#"
        {
            "contributors": 123
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    if let Ok(package_json_parser) = res {
        assert!(package_json_parser.validate().is_err());
    }
}
