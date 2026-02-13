use package_json_parser::PackageJsonParser;

#[test]
fn should_pass_when_keywords_is_valid() {
  let raw = [
    (
      r#"
        {
            "keywords": "test"
        }
    "#,
      0,
    ),
    (
      r#"
        {
            "keywords": ["test", "test2"]
        }
    "#,
      1,
    ),
  ];

  for (raw, expected_case) in raw {
    let res = PackageJsonParser::parse_str(raw);

    assert!(res.is_ok());

    if let Ok(package_json_parser) = res {
      let report = package_json_parser.validate().unwrap();
      assert!(!report.has_errors());
      let keywords = package_json_parser.get_keywords();
      let text = format!("{:?}", keywords.value);
      match expected_case {
        0 => assert!(text.contains("String(\"test\")")),
        1 => assert!(text.contains("Array([\"test\", \"test2\"])")),
        _ => unreachable!(),
      }
      assert!(!keywords.has_errors());
    }
  }
}

#[test]
fn should_fail_when_keywords_is_invalid() {
  let raw = r#"
        {
            "keywords": 123
        }
    "#;

  let res = PackageJsonParser::parse_str(raw);

  if let Ok(package_json_parser) = res {
    assert!(package_json_parser.validate().unwrap().has_errors());
  }
}
