use package_json_parser::PackageJsonParser;
use package_json_parser::Main;

#[test]
fn should_pass_when_main_is_valid() {
  let raw = [
    (
      r#"
        {
            "main": "test"
        }
    "#,
      0,
    ),
    (
      r#"
        {
            "main": false
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
      let main = package_json_parser.get_main();
      match expected_case {
        0 => assert!(matches!(main.value, Some(Main::Str(ref s)) if s == "test")),
        1 => assert!(matches!(main.value, Some(Main::Bool(false)))),
        _ => unreachable!(),
      }
      assert!(!main.has_errors());
    }
  }
}

#[test]
fn should_fail_when_main_is_invalid() {
  let raw = r#"
        {
            "main": 123
        }
    "#;

  let res = PackageJsonParser::parse_str(raw).unwrap();
  let report = res.validate().unwrap();
  assert!(report.has_errors());

  let main = res.get_main();
  assert!(main.value.is_none());
  assert!(main.has_errors());
}
