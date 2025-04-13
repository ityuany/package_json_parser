use package_json_parser::{PackageJsonParser, PublishConfig, Validate};

#[test]
fn should_pass_when_publish_config_is_valid() {
    let raw = r#"
        {
            "publishConfig": {
                "access": "public"
            }
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(
            package_json_parser.publish_config,
            Some(PublishConfig {
                access: Some("public".to_string()),
                registry: None,
                tag: None,
                provenance: None,
            })
        );

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_fail_when_publish_config_is_invalid() {
    let raw = r#"
        {
            "publishConfig": "invalid"
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    if let Ok(package_json_parser) = res {
        let res = package_json_parser.validate();
        assert!(res.is_err());
    }
}

#[test]
fn should_pass_when_publish_config_is_valid_with_registry() {
    let raw = r#"
        {
            "publishConfig": {
                "registry": "https://registry.npmjs.org"
            }
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(
            package_json_parser.publish_config,
            Some(PublishConfig {
                access: None,
                registry: Some("https://registry.npmjs.org".to_string()),
                tag: None,
                provenance: None,
            })
        );

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_fail_when_publish_config_is_invalid_with_registry() {
    let raw = r#"
        {
            "publishConfig": {
                "registry": "invalid"
            }
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    if let Ok(package_json_parser) = res {
        let res = package_json_parser.validate();
        assert!(res.is_err());
    }
}

#[test]
fn should_pass_when_publish_config_is_valid_with_tag() {
    let raw = r#"
        {
            "publishConfig": {
                "tag": "latest"
            }
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(
            package_json_parser.publish_config,
            Some(PublishConfig {
                access: None,
                registry: None,
                tag: Some("latest".to_string()),
                provenance: None,
            })
        );

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_fail_when_publish_config_is_invalid_with_tag() {
    let raw = r#"
        {
            "publishConfig": {
                "tag": "alpha"
            }
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    if let Ok(package_json_parser) = res {
        let res = package_json_parser.validate();
        assert!(res.is_ok());
    }
}

#[test]
fn should_pass_when_publish_config_is_valid_with_provenance() {
    let raw = r#"
        {
            "publishConfig": {
                "provenance": true
            }
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());
}
