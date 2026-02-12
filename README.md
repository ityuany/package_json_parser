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
  - If parsing fails, return `Err` (fatal).
- `validate` / `validate_with`:
  - Run semantic package.json validation.
  - Return `ValidationReport` (`errors` + `warnings`) when validation succeeds.
  - Return `Err` only for fatal cases (for example parse/diagnostic internal failures).

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

### Validation Policies

```rust
use package_json_parser::{
    PackageJsonParser,
    ValidationField,
    ValidationOptions,
    ValidationSeverity,
};

fn main() -> package_json_parser::Result<()> {
    let invalid_json = r#"
    {
        "name": "MyPackage",
        "version": "invalid-version",
        "bugs": "not-a-url-or-email"
    }
    "#;
    let package = PackageJsonParser::parse_str(invalid_json)?;

    // 1) Default policy: warning mode
    let warning_report = package.validate()?;
    assert_eq!(warning_report.errors.len(), 0);
    assert!(!warning_report.warnings.is_empty());

    // 2) Global error mode (suitable for CI blocking)
    let error_report = package.validate_with(ValidationOptions::error())?;
    assert!(error_report.has_errors());

    // 3) Global + field override
    let options = ValidationOptions::warning()
        .with(ValidationField::Name, ValidationSeverity::Error)
        .with(ValidationField::License, ValidationSeverity::Warning);
    let mixed_report = package.validate_with(options)?;
    assert!(mixed_report
        .errors
        .iter()
        .any(|issue| issue.field == ValidationField::Name));

    Ok(())
}
```

### Consuming `ValidationReport`

```rust
use package_json_parser::{PackageJsonParser, ValidationOptions};

fn main() -> package_json_parser::Result<()> {
    let package = PackageJsonParser::parse("package.json")?;
    let report = package.validate_with(ValidationOptions::error())?;

    for issue in &report.errors {
        println!(
            "[ERROR] field={:?} path={} message={}",
            issue.field, issue.json_path, issue.message
        );
    }
    for issue in &report.warnings {
        println!(
            "[WARN ] field={:?} path={} message={}",
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
use package_json_parser::{PackageJsonParser, ValidationOptions};

fn has_blocking_issues_for_ci(path: &str) -> package_json_parser::Result<bool> {
    let pkg = PackageJsonParser::parse(path)?;
    let report = pkg.validate_with(ValidationOptions::error())?;
    Ok(report.has_errors())
}

fn validate_for_local_dev(path: &str) -> package_json_parser::Result<()> {
    let pkg = PackageJsonParser::parse(path)?;
    let report = pkg.validate()?; // warning policy
    for w in &report.warnings {
        eprintln!("[warn] {}: {}", w.json_path, w.message);
    }
    Ok(())
}
```

## Documentation

For detailed documentation, please visit [docs.rs](https://docs.rs/package_json_parser).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 
