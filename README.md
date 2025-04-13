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
        }
        Err(e) => println!("Error parsing package.json: {}", e),
    }
}
```

## Documentation

For detailed documentation, please visit [docs.rs](https://docs.rs/package_json_parser).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 