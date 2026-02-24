# Agent.md

## 项目简介
- 名称：`package_json_parser`
- 类型：Rust 库（crate），用于解析并校验 `package.json`
- 当前版本：`0.0.16`
- Rust Edition：`2024`
- 许可证：`MIT`
- 仓库：<https://github.com/ityuany/package_json_parser>

## 核心能力
- 通过 `PackageJsonParser::parse(path)` / `PackageJsonParser::parse_str(content)` 解析 `package.json`
- 使用强类型结构体承载常见字段（如 `name`、`version`、`dependencies`、`scripts` 等）
- 通过 `validate()` 执行字段级校验，并基于 `miette` 提供可读错误诊断
- 支持 `bin_to_hash_map()` 将 `bin` 字段归一化为 `HashMap<String, String>`

## 代码结构
- `src/lib.rs`：对外主入口，定义 `PackageJsonParser` 与主要 API
- `src/def/`：`package.json` 各字段的数据结构与校验逻辑
- `src/ext/`：校验扩展能力（如 `Validator` trait 与诊断辅助）
- `src/err.rs`：错误类型定义
- `tests/`：集成测试
- `fixtures/`：测试样例 JSON
- `schema.json`：JSON schema 参考文件

## 关键依赖
- `serde` / `serde_json`：序列化与 JSON 解析
- `miette`：诊断与错误展示
- `thiserror`：错误类型建模
- `validator`：字段校验
- `jsonc-parser`：用于校验阶段 AST 辅助定位
- `rustc-hash`：高性能哈希结构

## 常用命令
- 格式化：`just fmt`
- 静态检查：`just lint`
- 测试：`just test`
- 一键检查：`just ready`
- 文档：`just doc`

## Agent 协作建议
- 修改字段定义时，优先在 `src/def/` 增量改动，并补充对应 `tests/` 用例
- 新增字段时，保持以下三处同步：
  1. `PackageJsonParser` 结构体字段
  2. `validate()` 中的 `validate_field!` 调用
  3. `src/def/` 的类型/校验实现与测试样例
- 对外错误信息优先保持 `miette` 可诊断格式，避免只返回裸字符串
- 提交前建议执行：`just ready && just test`

## `src/def/*` 手写反序列化参考范式（推荐）
- 适用场景：字段存在联合类型（如 `string | object`）、需要精确控制缺失/重复字段错误、需要为后续校验保留一致语义。
- 总体原则：反序列化只负责“类型结构正确”；业务合法性（格式、正则、URL、email）放在 `Validator::validate` 中处理。

### 1) 联合类型用外层 `Visitor`
- 典型形式：`enum X { String(String), Object(XObject) }`。
- 在 `impl Deserialize for X` 中：
  - `visit_str` / `visit_string` => `X::String(...)`
  - `visit_map` => `XObject::deserialize(MapAccessDeserializer::new(map))`
  - 入口使用 `deserializer.deserialize_any(VisitorImpl)`
- 目的：替代 `#[serde(untagged)]`，把行为显式化，便于调试和扩展。

### 2) 对象类型用内层 `Field + visit_map`
- 为对象定义 `enum Field { A, B, C, Ignore }`，并实现 `Deserialize` 把 key 映射到字段分支。
- 在 `visit_map` 中手动读取每个字段：
  - 必填字段：`Option<T>` 收集，最后用 `missing_field(...)` 校验
  - 可选字段：`Option<T>` 收集
  - 重复字段：使用 `de::Error::duplicate_field(...)`
  - 未知字段：`let _: IgnoredAny = map.next_value()?;` 显式消费
- 入口使用 `deserializer.deserialize_struct("TypeName", FIELDS, VisitorImpl)`。

### 3) 错误策略（和 serde 行为对齐）
- 类型不匹配：交给 `expecting(...)` + serde 默认报错路径。
- 字段重复：`de::Error::duplicate_field("field")`。
- 必填缺失：`de::Error::missing_field("field")`。
- 未知字段：默认忽略（除非该字段明确要求严格拒绝）。

### 4) 测试清单（每个 def 建议最少覆盖）
- 解析成功：
  - 每个合法输入分支都可 `parse_str` 成功。
- 解析失败：
  - 错误值类型（如期望 `string|object` 却给 `number/bool/array`）返回 `Err`。
  - 必填字段缺失返回 `Err`。
  - 重复字段返回 `Err`（若实现了重复检测）。
- 校验成功/失败：
  - `parse_str` 成功后，`validate()` 对业务规则分别覆盖通过与失败样例。

### 5) 当前落地样例
- 参考实现：`src/def/person.rs`
- 该文件已演示：
  - `Person`（联合类型）手写 `Deserialize`
  - `PersonObject`（对象）手写 `Deserialize`
  - 解析阶段类型错误测试 + validate 阶段业务规则测试

## 当前状态（初始化记录）
- 已创建 `agent.md`
- 初始化时间：2026-02-24
