// 使用 schemafy_lib 宏从 schema 文件生成 Rust 结构体

// 从文件生成结构体
// schemafy_lib::schema!("src/schema.json");

use package_json_parser::PackageJsonParser;

// 现在可以使用生成的结构体
fn main() {
    let package_json_parser = PackageJsonParser::parse(
        "/Users/ityuany/GitRepository/doctor-engine/napi/doctor_engine/package.json",
    )
    .unwrap();

    println!("{:#?}", package_json_parser);
}
