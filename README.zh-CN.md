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
package_json_parser = "0.0.1"
```

## 使用方法

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
            println!("包名: {}", package.name);
            println!("版本: {}", package.version);
            
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
use package_json_parser::PackageJson;

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

    let package = PackageJson::from_str(valid_json).unwrap();
    assert!(package.validate().is_ok());

    // 验证无效的 package.json
    let invalid_json = r#"
    {
        "name": "my-package",
        "version": "invalid-version",  // 无效的版本号
        "description": 123,            // 无效的类型
        "main": true                   // 无效的类型
    }
    "#;

    let package = PackageJson::from_str(invalid_json).unwrap();
    if let Err(e) = package.validate() {
        println!("验证错误: {}", e);
        // 输出类似:
        // 验证错误: version: 无效的版本号格式
        // description: 必须是字符串
        // main: 必须是字符串
    }
}
```

## 文档

详细文档请访问 [docs.rs](https://docs.rs/package_json_parser)。

## 贡献

欢迎贡献代码！请随时提交 Pull Request。

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。 