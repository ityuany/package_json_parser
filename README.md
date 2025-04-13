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
package_json_parser = "0.0.1"
```

## Usage

```rust
use package_json_parser::PackageJson;

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

    match PackageJson::from_str(json_str) {
        Ok(package) => {
            println!("Package name: {}", package.name);
            println!("Version: {}", package.version);
            
            // Validate package.json
            match package.validate() {
                Ok(_) => println!("package.json validation passed"),
                Err(e) => println!("package.json validation failed: {}", e),
            }
        }
        Err(e) => println!("Error parsing package.json: {}", e),
    }
}
```

### Validation Examples

```rust
use package_json_parser::PackageJson;

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

    let package = PackageJson::from_str(valid_json).unwrap();
    assert!(package.validate().is_ok());

    // Validate an invalid package.json
    let invalid_json = r#"
    {
        "name": "my-package",
        "version": "invalid-version",  // invalid version format
        "description": 123,            // invalid type
        "main": true                   // invalid type
    }
    "#;

    let package = PackageJson::from_str(invalid_json).unwrap();
    if let Err(e) = package.validate() {
        println!("Validation errors: {}", e);
        // Output similar to:
        // Validation errors: version: invalid version format
        // description: must be a string
        // main: must be a string
    }
}
```

## Documentation

For detailed documentation, please visit [docs.rs](https://docs.rs/package_json_parser).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 