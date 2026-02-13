use package_json_parser::PackageJsonParser;

fn rendered_parse_error(raw: &str) -> String {
  let pkg = PackageJsonParser::parse_str(raw).unwrap();
  let report = pkg.validate().unwrap();
  assert!(!report.errors.is_empty());
  pkg.render_issue(&report.errors[0])
}

#[test]
fn main_type_error_message_is_explicit() {
  let text = rendered_parse_error(r#"{ "main": 123 }"#);
  assert!(text.contains("a boolean or string for `main`"));
}

#[test]
fn bin_type_error_message_is_explicit() {
  let text = rendered_parse_error(r#"{ "bin": 123 }"#);
  assert!(text.contains("expected a string or object map"));
  assert!(text.contains("for `bin`"));
}

#[test]
fn bugs_type_error_message_is_explicit() {
  let text = rendered_parse_error(r#"{ "bugs": 123 }"#);
  assert!(text.contains("a string (url/email) or object"));
  assert!(text.contains("for `bugs`"));
}

#[test]
fn repository_type_error_message_is_explicit() {
  let text = rendered_parse_error(r#"{ "repository": 123 }"#);
  assert!(text.contains("a string or object"));
  assert!(text.contains("for `repository`"));
}

#[test]
fn keywords_type_error_message_is_explicit() {
  let text = rendered_parse_error(r#"{ "keywords": 123 }"#);
  assert!(text.contains("a string or array of strings"), "{text}");
  assert!(text.contains("`keywords`"), "{text}");
}

#[test]
fn readme_type_error_message_is_explicit() {
  let text = rendered_parse_error(r#"{ "readme": 123 }"#);
  assert!(text.contains("a string or object"));
  assert!(text.contains("for `readme`"));
}

#[test]
fn person_type_error_message_is_explicit() {
  let text = rendered_parse_error(r#"{ "author": 123 }"#);
  assert!(text.contains("a string or object"), "{text}");
  assert!(text.contains("\"name\":"), "{text}");
  assert!(text.contains("\"url\"?: string"), "{text}");
}
