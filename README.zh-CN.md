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

### 核心模型

- `parse_str` / `parse`：
  - 负责把 JSON 解析为 `PackageJsonParser`。
  - JSON 语法错误或读取失败会直接返回 `Err`（致命错误）。
- `validate` / `validate_with`：
  - 负责做 package.json 语义规则校验。
  - 返回 `ValidationReport`（包含 `errors` + `warnings`）。
  - 仅在致命异常（例如内部解析/状态异常）时返回 `Err`。

### 快速开始

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

### 校验策略示例

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

    // 1) 默认策略：warning
    let warning_report = package.validate()?;
    assert_eq!(warning_report.errors.len(), 0);
    assert!(!warning_report.warnings.is_empty());

    // 2) 全局 error（适合 CI 阻断）
    let error_report = package.validate_with(ValidationOptions::error())?;
    assert!(error_report.has_errors());

    // 3) 全局 + 字段覆盖
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

### 如何消费 `ValidationReport`

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
        // 由调用方决定是否阻断流程（构建、发布、提交等）
    }

    Ok(())
}
```

### 典型接入方式

```rust
use package_json_parser::{PackageJsonParser, ValidationOptions};

fn has_blocking_issues_for_ci(path: &str) -> package_json_parser::Result<bool> {
    let pkg = PackageJsonParser::parse(path)?;
    let report = pkg.validate_with(ValidationOptions::error())?;
    Ok(report.has_errors())
}

fn validate_for_local_dev(path: &str) -> package_json_parser::Result<()> {
    let pkg = PackageJsonParser::parse(path)?;
    let report = pkg.validate()?; // warning 策略
    for w in &report.warnings {
        eprintln!("[warn] {}: {}", w.json_path, w.message);
    }
    Ok(())
}
```

## 文档

详细文档请访问 [docs.rs](https://docs.rs/package_json_parser)。

## 贡献

欢迎贡献代码！请随时提交 Pull Request。

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。 
