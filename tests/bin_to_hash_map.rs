use package_json_parser::{ErrorKind, PackageJsonParser};

#[test]
fn should_pass_when_bin_is_valid() {
  let raw = r#"
        {
            "name": "test",
            "bin": {
                "test": "test"
            }
        }
    "#;
  let package_json_parser = serde_json::from_str::<PackageJsonParser>(raw);

  assert!(package_json_parser.is_ok());

  if let Ok(package_json_parser) = package_json_parser {
    let bin = package_json_parser.bin_to_hash_map();
    assert!(bin.is_ok());
    if let Ok(bin) = bin {
      assert_eq!(bin.len(), 1);
      assert_eq!(bin.get("test"), Some(&"test".to_string()));
    }
  }
}

#[test]
fn should_fail_when_bin_is_invalid() {
  let raw = r#"
        {
            "bin": "test"
        }
    "#;
  let package_json_parser = serde_json::from_str::<PackageJsonParser>(raw);

  assert!(package_json_parser.is_ok());

  if let Ok(package_json_parser) = package_json_parser {
    let bin = package_json_parser.bin_to_hash_map();
    assert!(bin.is_err());
    if let Err(e) = bin {
      assert!(matches!(e, ErrorKind::NameRequired));
    }
  }
}

#[test]
fn should_pass_when_bin_is_valid_with_name() {
  let raw = r#"
        {
            "name": "@onecoc/uni",
            "bin": "./uni.js"
        }
    "#;
  let package_json_parser = serde_json::from_str::<PackageJsonParser>(raw);

  assert!(package_json_parser.is_ok());

  if let Ok(package_json_parser) = package_json_parser {
    let bin = package_json_parser.bin_to_hash_map();
    assert!(bin.is_ok());
    if let Ok(bin) = bin {
      assert_eq!(bin.len(), 1);
      assert_eq!(bin.get("uni"), Some(&"./uni.js".to_string()));
    }
  }
}
