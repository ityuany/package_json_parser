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

```rust
use package_json_parser::PackageJsonParser;

fn main() {
    let json_str = r#"
    {
        "name": "my-package",
        "version": "1.0.0",
        "dependencies": {
            "some-package": "^1.0.0"
        }
    }
    "#;

    match PackageJsonParser::parse_str(json_str) {
        Ok(package) => {
            if let Some(name) = package.name.as_ref() {
                println!("Package name: {}", name.as_str());
            }
            if let Some(version) = package.version.as_ref() {
                println!("Version: {}", version.as_str());
            }
            
            // Validate package.json
            match package.validate() {
                Ok(report) => {
                    println!("errors: {}", report.errors.len());
                    println!("warnings: {}", report.warnings.len());
                }
                Err(e) => println!("package.json validation failed: {}", e),
            };
        }
        Err(e) => println!("Error parsing package.json: {}", e),
    }
}
```

### Validation Examples

```rust
use package_json_parser::{
    PackageJsonParser,
    ValidationField,
    ValidationOptions,
    ValidationSeverity,
};

fn main() {
    // Validate a valid package.json
    let valid_json = r#"
    {
        "name": "my-package",
        "version": "1.0.0",
        "description": "A test package",
        "main": "index.js",
        "scripts": {
            "test": "echo \"Error: no test specified\" && exit 1"
        },
        "keywords": ["test"],
        "author": "Test User",
        "license": "MIT"
    }
    "#;

    let package = PackageJsonParser::parse_str(valid_json).unwrap();
    let report = package.validate().unwrap();
    assert!(report.is_clean());

    // Validate an invalid package.json (JSON is valid, but fields violate package.json rules)
    let invalid_json = r#"
    {
        "name": "MyPackage",
        "version": "invalid-version",
        "bugs": "not-a-url-or-email"
    }
    "#;

    let package = PackageJsonParser::parse_str(invalid_json).unwrap();

    // 1) Default mode (lenient): violations are warnings
    let report = package.validate().unwrap();
    assert_eq!(report.errors.len(), 0);
    assert!(report.warnings.len() >= 1);

    // 2) Strict mode: violations become errors
    let report = package.validate_strict().unwrap();
    assert!(report.has_errors());

    // 3) Global + field override:
    //    global Warning, but `name` is forced to Error
    let options = ValidationOptions::lenient()
        .field(ValidationField::Name, ValidationSeverity::Error);
    let report = package.validate_with(options).unwrap();
    assert!(report.errors.iter().any(|issue| issue.field == ValidationField::Name));
}
```

### Migration Note

- Before `v0.0.16`: `validate()` returned `Result<()>` and failed on first violation.
- Since `v0.0.16`: `validate()` returns `Result<ValidationReport>` and defaults to lenient mode.
- To keep blocking behavior, use `validate_strict()` and check `report.has_errors()`.

## Documentation

For detailed documentation, please visit [docs.rs](https://docs.rs/package_json_parser).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 
