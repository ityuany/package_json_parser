# package_json_parser

[![Crates.io](https://img.shields.io/crates/v/package_json_parser.svg)](https://crates.io/crates/package_json_parser)
[![Documentation](https://docs.rs/package_json_parser/badge.svg)](https://docs.rs/package_json_parser)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library for parsing and validating package.json files.

## Features

- Parse package.json files with strong type safety
- Validate package.json fields according to npm specifications
- Error handling with detailed error messages
- Efficient parsing with optimized dependencies

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
package_json_parser = "0.0.16"
```

## Usage

### Core Model

- `parse_str` / `parse`:
  - Parse JSON into `PackageJsonParser`.
  - Fail only on fatal errors (I/O or JSON syntax).
- `validate`:
  - Collect all issues in one pass and return `ValidationReport`.
  - This version uses a fixed `Error` severity model (no policy switching).
- `get_xxx`:
  - Return `FieldResult<T> { value, issues }`.
  - Useful for business-path field access.

### Pretty Fatal Error Output

`PackageJsonError` now implements `miette::Diagnostic`, so callers can render fatal errors directly:

```rust
use package_json_parser::PackageJsonParser;

fn main() {
    if let Err(err) = PackageJsonParser::parse("package.json") {
        eprintln!("{:?}", miette::Report::new(err));
    }
}
```

### Quick Start

```rust
use package_json_parser::PackageJsonParser;

fn main() -> package_json_parser::Result<()> {
    let json_str = r#"
    {
        "name": "my-package",
        "version": "1.0.0",
        "dependencies": {
            "some-package": "^1.0.0"
        }
    }
    "#;

    let package = PackageJsonParser::parse_str(json_str)?;
    let report = package.validate()?;

    println!("errors: {}", report.errors.len());
    println!("warnings: {}", report.warnings.len());

    Ok(())
}
```

### Validation

```rust
use package_json_parser::PackageJsonParser;

fn main() -> package_json_parser::Result<()> {
    let invalid_json = r#"
    {
        "name": "MyPackage",
        "version": "invalid-version",
        "bugs": "not-a-url-or-email"
    }
    "#;
    let package = PackageJsonParser::parse_str(invalid_json)?;

    let report = package.validate()?;
    assert!(report.has_errors());
    assert!(report.warnings.is_empty());

    Ok(())
}
```

### Consuming `ValidationReport`

```rust
use package_json_parser::PackageJsonParser;

fn main() -> package_json_parser::Result<()> {
    let package = PackageJsonParser::parse("package.json")?;
    let report = package.validate()?;

    for issue in &report.errors {
        println!(
            "[ERROR] field={:?} path={} message={}",
            issue.field, issue.json_path, issue.message
        );
    }

    if report.has_errors() {
        // decide whether to block build/publish
    }

    Ok(())
}
```

### Typical Integration Patterns

```rust
use package_json_parser::PackageJsonParser;

fn has_blocking_issues_for_ci(path: &str) -> package_json_parser::Result<bool> {
    let pkg = PackageJsonParser::parse(path)?;
    let report = pkg.validate()?;
    Ok(report.has_errors())
}

fn validate_for_local_dev(path: &str) -> package_json_parser::Result<()> {
    let pkg = PackageJsonParser::parse(path)?;
    let report = pkg.validate()?;
    for e in &report.errors {
        eprintln!("[error] {}: {}", e.json_path, e.message);
    }
    Ok(())
}
```

### Field Access

```rust
use package_json_parser::PackageJsonParser;

fn main() -> package_json_parser::Result<()> {
    let pkg = PackageJsonParser::parse_str(r#"{ "main": 123 }"#)?;
    let main = pkg.get_main();

    assert!(main.value.is_none());
    assert!(main.has_errors());

    Ok(())
}
```

## Documentation

For detailed documentation, please visit [docs.rs](https://docs.rs/package_json_parser).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 
