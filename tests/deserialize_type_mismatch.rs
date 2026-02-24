use package_json_parser::PackageJsonParser;

#[test]
fn should_fail_parse_when_field_type_mismatches() {
  let cases = [
    (r#"{ "name": 1 }"#, "name"),
    (r#"{ "version": true }"#, "version"),
    (r#"{ "description": 1 }"#, "description"),
    (r#"{ "keywords": 1 }"#, "keywords"),
    (r#"{ "homepage": 1 }"#, "homepage"),
    (r#"{ "bugs": false }"#, "bugs"),
    (r#"{ "license": 1 }"#, "license"),
    (r#"{ "author": 1 }"#, "author"),
    (r#"{ "contributors": 1 }"#, "contributors"),
    (r#"{ "maintainers": 1 }"#, "maintainers"),
    (r#"{ "files": 1 }"#, "files"),
    (r#"{ "main": 1 }"#, "main"),
    (r#"{ "type": 1 }"#, "type"),
    (r#"{ "types": 1 }"#, "types"),
    (r#"{ "typings": 1 }"#, "typings"),
    (r#"{ "packageManager": 1 }"#, "packageManager"),
    (r#"{ "publishConfig": "invalid" }"#, "publishConfig"),
    (
      r#"{ "publishConfig": { "provenance": "true" } }"#,
      "publishConfig.provenance",
    ),
    (r#"{ "bin": 1 }"#, "bin"),
    (r#"{ "man": 1 }"#, "man"),
    (r#"{ "directories": 1 }"#, "directories"),
    (r#"{ "repository": 1 }"#, "repository"),
    (r#"{ "module": 1 }"#, "module"),
    (r#"{ "readme": 1 }"#, "readme"),
    (
      r#"{ "readme": { "type": 1, "value": "x" } }"#,
      "readme.type",
    ),
    (r#"{ "private": "true" }"#, "private"),
    (r#"{ "engines": 1 }"#, "engines"),
    (r#"{ "engineStrict": "true" }"#, "engineStrict"),
    (r#"{ "os": 1 }"#, "os"),
    (r#"{ "cpu": 1 }"#, "cpu"),
    (r#"{ "scripts": 1 }"#, "scripts"),
    (r#"{ "dependencies": 1 }"#, "dependencies"),
    (r#"{ "devDependencies": 1 }"#, "devDependencies"),
    (r#"{ "optionalDependencies": 1 }"#, "optionalDependencies"),
    (r#"{ "peerDependencies": 1 }"#, "peerDependencies"),
  ];

  for (raw, field) in cases {
    let res = PackageJsonParser::parse_str(raw);
    assert!(
      res.is_err(),
      "expected parse error for field `{field}`: {raw}"
    );
  }
}
