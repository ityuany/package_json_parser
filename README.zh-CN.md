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
            if let Ok(Some(name)) = package.name() {
                println!("包名: {}", name.as_str());
            }
            if let Ok(Some(version)) = package.version() {
                println!("版本: {}", version.as_str());
            }
            
            // 验证 package.json
            match package.validate() {
                Ok(_) => println!("package.json 验证通过"),
                Err(e) => println!("package.json 验证失败: {}", e),
            }
        }
        Err(e) => println!("解析 package.json 时出错: {}", e),
    }
}
```

### 验证示例

```rust
use package_json_parser::PackageJsonParser;

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
    assert!(package.validate().is_ok());

    // 验证无效的 package.json（JSON 语法合法，但字段不符合 package.json 规则）
    let invalid_json = r#"
    {
        "name": "MyPackage",
        "version": "invalid-version",
        "bugs": "not-a-url-or-email"
    }
    "#;

    let package = PackageJsonParser::parse_str(invalid_json).unwrap();
    if let Err(e) = package.validate() {
        println!("验证错误: {}", e);
        // 输出类似:
        // 验证错误: 包名不符合要求的正则规则
        // 验证错误: version 字段格式非法
        // 验证错误: bugs 不是合法 URL 或邮箱
    }
}
```

## 文档

详细文档请访问 [docs.rs](https://docs.rs/package_json_parser)。

## 贡献

欢迎贡献代码！请随时提交 Pull Request。

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。 
