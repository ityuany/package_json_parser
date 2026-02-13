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
  - 仅在致命错误（IO / JSON 语法）时返回 `Err`。
- `validate`：
  - 一次性收集全部问题并返回 `ValidationReport`。
  - 当前版本固定为 `Error` 模型，不支持策略切换。
- `get_xxx`：
  - 返回 `FieldResult<T> { value, issues }`，用于业务读取路径。

### 致命错误的漂亮输出

`PackageJsonError` 已实现 `miette::Diagnostic`，调用方可以直接渲染诊断信息：

```rust
use package_json_parser::PackageJsonParser;

fn main() {
    if let Err(err) = PackageJsonParser::parse("package.json") {
        eprintln!("{:?}", miette::Report::new(err));
    }
}
```

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

### 校验示例

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

### 如何消费 `ValidationReport`

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
        // 由调用方决定是否阻断流程（构建、发布、提交等）
    }

    Ok(())
}
```

### 典型接入方式

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

### 字段读取

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

## 文档

详细文档请访问 [docs.rs](https://docs.rs/package_json_parser)。

## 贡献

欢迎贡献代码！请随时提交 Pull Request。

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。 
