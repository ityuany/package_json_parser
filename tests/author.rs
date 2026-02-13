use package_json_parser::{PackageJsonParser, Person, PersonObject};

#[test]
fn should_pass_when_author_is_valid() {
  let raw = r#"
        {
            "author": "test"
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  assert!(res.is_ok());

  if let Ok(package_json_parser) = res {
    assert_eq!(
      package_json_parser.author,
      Some(Person::String("test".to_string()))
    );

    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
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

  let res = PackageJsonParser::parse_str(raw);

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

    let report = package_json_parser.validate().unwrap();
    assert!(!report.has_errors());
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

  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    assert!(package_json_parser.validate().unwrap().has_errors());
  }
}
