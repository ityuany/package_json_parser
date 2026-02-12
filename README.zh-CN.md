# package_json_parser

[![Crates.io](https://img.shields.io/crates/v/package_json_parser.svg)](https://crates.io/crates/package_json_parser)
[![Documentation](https://docs.rs/package_json_parser/badge.svg)](https://docs.rs/package_json_parser)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个用于解析和验证 package.json 文件的 Rust 库。

## 特性

- 使用强类型安全的方式解析 package.json 文件
- 根据 npm 规范验证 package.json 字段
- 提供详细的错误信息处理
- 使用优化的依赖项实现高效解析

## 安装

在 `Cargo.toml` 中添加以下内容：

```toml
[dependencies]
package_json_parser = "0.0.16"
```

## 使用方法

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
                println!("包名: {}", name.as_str());
            }
            if let Some(version) = package.version.as_ref() {
                println!("版本: {}", version.as_str());
            }
            
            // 验证 package.json
            match package.validate() {
                Ok(report) => {
                    println!("errors: {}", report.errors.len());
                    println!("warnings: {}", report.warnings.len());
                }
                Err(e) => println!("package.json 验证失败: {}", e),
            };
        }
        Err(e) => println!("解析 package.json 时出错: {}", e),
    }
}
```

### 验证示例

```rust
use package_json_parser::{
    PackageJsonParser,
    ValidationField,
    ValidationOptions,
    ValidationSeverity,
};

fn main() {
    // 验证有效的 package.json
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

    // 验证无效的 package.json（JSON 语法合法，但字段不符合 package.json 规则）
    let invalid_json = r#"
    {
        "name": "MyPackage",
        "version": "invalid-version",
        "bugs": "not-a-url-or-email"
    }
    "#;

    let package = PackageJsonParser::parse_str(invalid_json).unwrap();

    // 1) 默认模式（宽松）：违规会进入 warnings
    let report = package.validate().unwrap();
    assert_eq!(report.errors.len(), 0);
    assert!(report.warnings.len() >= 1);

    // 2) 严格模式：违规会进入 errors
    let report = package.validate_strict().unwrap();
    assert!(report.has_errors());

    // 3) 全局 + 字段覆盖：
    //    全局 Warning，但 name 强制为 Error
    let options = ValidationOptions::lenient()
        .field(ValidationField::Name, ValidationSeverity::Error);
    let report = package.validate_with(options).unwrap();
    assert!(report.errors.iter().any(|issue| issue.field == ValidationField::Name));
}
```

### 迁移说明

- `v0.0.16` 之前：`validate()` 返回 `Result<()>`，遇到首个违规即返回失败。
- `v0.0.16` 起：`validate()` 返回 `Result<ValidationReport>`，默认是宽松模式。
- 如果你需要“发现违规就阻断”，请使用 `validate_strict()` 并检查 `report.has_errors()`。

## 文档

详细文档请访问 [docs.rs](https://docs.rs/package_json_parser)。

## 贡献

欢迎贡献代码！请随时提交 Pull Request。

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。 
