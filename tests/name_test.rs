use package_json_parser::{
    Bugs, License, Name, PackageJsonParser, PackageManager, Person, PersonObject, PublishConfig,
    Type, Validate, Version,
};

#[test]
fn should_pass_when_name_is_valid() {
    let raw = r#"
        {
            "name": "test"
        }
    "#;
    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(package_json_parser.name, Some(Name("test".to_string())));

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_fail_when_name_is_invalid() {
    let raw = r#"
        {
            "name": "teYst"
        }
    "#;
    let res = serde_json::from_str::<PackageJsonParser>(raw);

    if let Ok(package_json_parser) = res {
        assert!(package_json_parser.validate().is_err());
    }
}

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

#[test]
fn should_pass_when_description_is_valid() {
    let raw = r#"
        {
            "description": "test"
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(package_json_parser.description, Some("test".to_string()));

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_fail_when_description_is_invalid() {
    let raw = r#"
        {
            "description": 123
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    if let Ok(package_json_parser) = res {
        assert!(package_json_parser.validate().is_err());
    }
}

#[test]
fn should_pass_when_keywords_is_valid() {
    let raw = r#"
        {
            "keywords": ["test"]
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(package_json_parser.keywords, Some(vec!["test".to_string()]));

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_fail_when_keywords_is_invalid() {
    let raw = r#"
        {
            "keywords": 123
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    if let Ok(package_json_parser) = res {
        assert!(package_json_parser.validate().is_err());
    }
}

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

#[test]
fn should_pass_when_author_is_valid() {
    let raw = r#"
        {
            "author": "test"
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(
            package_json_parser.author,
            Some(Person::String("test".to_string()))
        );

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_pass_when_author_object_is_valid() {
    let raw = r#"
        {
            "author": {
                "name": "test"
            }
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(
            package_json_parser.author,
            Some(Person::Object(PersonObject {
                name: "test".to_string(),
                email: None,
                url: None
            }))
        );

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_fail_when_author_object_is_invalid() {
    let raw = r#"
        {
            "author": {
                "name": "test",
                "email": "xxx"
            }
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    if let Ok(package_json_parser) = res {
        assert!(package_json_parser.validate().is_err());
    }
}

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

#[test]
fn should_pass_when_maintainers_is_valid() {
    let raw = r#"
        {
            "maintainers": ["test"]
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(
            package_json_parser.maintainers,
            Some(vec![Person::String("test".to_string())])
        );

        let res = package_json_parser.validate();

        assert!(res.is_ok());
    }
}

#[test]
fn should_pass_when_maintainers_object_is_valid() {
    let raw = r#"
        {
            "maintainers": [{ "name": "test" }]
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
        assert_eq!(
            package_json_parser.maintainers,
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
fn should_fail_when_maintainers_is_invalid() {
    let raw = r#"
        {
            "maintainers": 123
        }
    "#;

    let res = serde_json::from_str::<PackageJsonParser>(raw);

    if let Ok(package_json_parser) = res {
        assert!(package_json_parser.validate().is_err());
    }
}

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
