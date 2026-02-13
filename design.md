# 设计概要：Raw-First Parser（重大重构版）

## 背景与目标

本次设计允许并接受超级大 break-change：重写现有 `PackageJsonParser`，不考虑平滑过渡。

目标：

1. `parse/parse_str` 只处理致命错误（IO、JSON 语法），不因字段类型错误失败。
2. `validate/validate_with` 统一收敛：
  - 字段类型错误（TypeMismatch）
  - 字段语义错误（SemanticViolation）
3. 字段读取 API 统一返回结构化结果：
  - `value`：字段值（可选）
  - `issues`：字段相关问题（含 severity）

## 决策状态

### 已锁定（来自当前决策）

1. 架构层面允许彻底重写 `PackageJsonParser`（重大 break-change）。
2. 字段读取语义锁定为 `FieldResult<T>`：
  - `value: Option<T>` 表示可用值（`None` 代表缺失或不可解码）。
  - `issues: Vec<ValidationIssue>` 承载字段问题（TypeMismatch / SemanticViolation）。
3. `validate` 与 `get_xxx` 必须共享同一套字段解码逻辑（单一真相源）。
4. 性能策略采用字段级缓存（lazy + cache），避免重复解析。
5. 错误定位采用 AST 方案（保留 span，提供高质量定位）。

### 待决策（TODO）

无（A/B 均已决策）。

## 新架构总览

`PackageJsonParser` 改为 raw-first：

1. `raw_source: String`：原始内容，用于诊断渲染。
2. `raw_json: serde_json::Value`：宽松读取与延迟解码输入。
3. `raw_ast: jsonc_parser::ast`（或等价对象）：用于精确 span/path 定位。
4. `decode_cache: HashMap<ValidationField, DecodeState>`：字段级解码缓存。

其中 `DecodeState` 建议：

- `Missing`
- `Ok(TypedValue)`
- `Err(FieldDecodeError)`

`TypedValue` 需要是可枚举联合类型，避免“不同字段类型无法放入同一个 cache”：

```rust
enum TypedValue {
  Name(Name),
  Version(Version),
  Main(Main),
  License(License),
  // ... 其它字段逐步补齐
}
```

说明：

1. 这就是 point #2 的核心：`HashMap<ValidationField, ...>` 的 value 必须能承载“多种字段类型”。
2. 如果不引入 `TypedValue` 联合类型，缓存将无法统一存储不同字段的解析结果。

## API 语义（重定义）

### parse / parse_str

- 成功条件：
  - IO 成功（`parse(path)`）
  - 标准 JSON 语法合法（仅标准 JSON，不支持 JSONC 扩展）
- 失败条件（fatal）：
  - IO 错误
  - JSON 语法错误

### validate / validate_with

- 行为：
  - 遍历所有受支持字段
  - 调用统一字段解码器
  - 收集 TypeMismatch + SemanticViolation
  - 统一产出 `ValidationReport`
- 返回：
  - `Ok(report)`：即使有大量问题也不报 `Err`
  - `Err`：仅内部不可恢复异常

默认策略说明（避免歧义）：

1. `validate()` 默认走 warning 策略。
2. `TypeMismatch` 与 `SemanticViolation` 都走同一套 severity 策略（全局 + 字段覆盖）。

### get_xxx

签名范式：`fn get_name(&self) -> FieldResult<Name>`

统一返回结构：

```rust
pub struct FieldResult<T> {
  pub value: Option<T>,
  pub issues: Vec<ValidationIssue>,
}
```

规则：

1. 字段不存在：`value = None`，`issues = []`。
2. 字段存在但类型不匹配：`value = None`，`issues` 里包含 `TypeMismatch`。
3. 字段类型正确但语义无效：`value = Some(T)` 或 `None`（由字段定义决定），并在 `issues` 添加 `SemanticViolation`。
4. 字段可用且无问题：`value = Some(T)`，`issues = []`。
5. 仅内部不可恢复异常才使用顶层 `Result`（如果实现保留该层）。

注：

- `get_xxx` 主定位是“业务读取路径”，强调字段级读取与字段级问题可见。
- `validate` 主定位是“质量治理路径”，强调全量收敛与规则一致性。

## 一致性约束（强约束）

所有字段都必须经过同一个字段解码器（示意）：

```rust
fn decode_name(raw_json: &Value, ast: &Ast) -> DecodeResult<Name>;
```

`validate` 和 `get_name` 只能调用该函数，不允许重复实现解析逻辑。

## 缓存与并发模型（实现约束）

`get_xxx` 与 `validate` 都会触发解码缓存写入，因此需要内部可变性。

建议实现：

1. 单线程优先：`RefCell<HashMap<ValidationField, DecodeState>>`
2. 并发安全：`RwLock<HashMap<ValidationField, DecodeState>>`
3. 惰性初始化：可配合 `OnceLock` 管理 cache 生命周期

要求：

- 文档与实现必须明确选择其中一种，不允许“隐式可变”。

## 校验收敛模型

统一 issue 结构，至少包含：

- `field`
- `kind`（`TypeMismatch` / `SemanticViolation`）
- `severity`
- `message`
- `help`
- `json_path`
- `span`

severity 分配规则：

- 当前版本固定 `Error` 分级（不提供策略切换）
- `TypeMismatch` 与 `SemanticViolation` 都归入 `errors`

## 使用分工（强烈建议）

- 业务读取路径：`get_xxx`
  - 场景：运行时只关心少数字段（如 `name/version/main`）。
  - 目标：按需读取，拿到字段值与字段问题（`FieldResult<T>`）。
- 质量治理路径：`validate`
  - 场景：CI、发布前检查、质量报告、治理看板。
  - 目标：一次性收敛全部问题（TypeMismatch + SemanticViolation）。

## 渲染策略（已决策，B1.5）

对外 API 只暴露结构化类型，不暴露 `miette` 类型：

- `ValidationIssue`
- `ValidationReport`
- `PackageJsonError`

提供字符串渲染能力作为默认用户体验：

- `render_issue(&self, issue: &ValidationIssue) -> String`
- `render_report(&self, report: &ValidationReport) -> String`

为避免 report 脱离 parser 后不便渲染，建议补充上下文对象：

- `render_context(&self) -> RenderContext`
- `RenderContext::render_issue(&self, issue: &ValidationIssue) -> String`
- `RenderContext::render_report(&self, report: &ValidationReport) -> String`

说明：

1. 内部可以使用 `miette` 生成高质量诊断文本。
2. 对外返回 `String`，避免调用方与 `miette` 版本耦合。
3. 调用方无需安装/关心 `miette` 也可得到漂亮日志。

## 典型调用流程（目标体验）

```rust
let pkg = PackageJsonParser::parse("package.json")?;

let report = pkg.validate()?;

if report.has_errors() {
  for issue in &report.errors {
    eprintln!("{}", pkg.render_issue(issue));
  }
}

if !report.warnings.is_empty() {
  for issue in &report.warnings {
    eprintln!("{}", pkg.render_issue(issue));
  }
}

let name = pkg.get_name();
if name.value.is_none() {
  for issue in &name.issues {
    eprintln!("{}", pkg.render_issue(issue));
  }
}
```

## 落地阶段

1. 阶段 1（核心骨架）
  - 重写 parser 为 raw-first。
  - 打通 AST + raw_json 双存储。
2. 阶段 2（统一解码链路）
  - 先实现 `name/version/main` 的 shared decode。
  - `validate` 和 `get_xxx` 全部走 shared decode。
3. 阶段 3（全字段覆盖）
  - 扩展所有字段。
  - 接入字段级 cache。
4. 阶段 4（诊断体验）
  - 完善 `help/code/span/json_path`。
  - 稳定 issue 渲染 API。

## 测试要求（必须满足）

1. 单字段类型错误不阻断 `parse`。
2. `validate` 能看到该类型错误。
3. 对同一字段：
  - `validate` 报错
  - `get_xxx` 在 `issues` 中可见同类问题
  - 两者 message/code 一致
4. 缺失字段：
  - `validate` 不报类型错误
  - `get_xxx` 返回 `value = None` 且 `issues = []`
5. 语义错误：
  - `parse` 成功
  - `validate` 报 `SemanticViolation`
6. span/path 正确指向字段位置。

## A 决策记录：固定 Error 分级（已采用）

TypeMismatch 与 SemanticViolation 均固定为 `Error`，不支持运行时策略切换。

选择理由：

1. API 更简单，无策略分支和行为歧义。
2. `validate` 与 `get_xxx` 的问题等级完全一致。
3. 治理路径默认可直接阻断，无需额外配置。

## B 决策记录：结构化主 API + 字符串渲染适配（已采用 B1.5）

采用策略：

1. 结构化类型作为主出口（供业务与治理系统消费）。
2. 提供字符串渲染 API 作为展示出口（CLI/日志直接可用）。
3. `miette` 仅在内部/适配层使用，不出现在公共 API 签名中。

## 解析一致性约束（raw_json vs raw_ast）

为保证逻辑自洽，需要明确两者来源与一致性策略：

1. 推荐：一次读取原文，分别构建 `raw_json` 与 `raw_ast`，并以同一份 `raw_source` 为输入源。
2. 字段值读取以 `raw_json` 为准；span/path 定位以 `raw_ast` 为准。
3. 若两者出现不一致（理论上不应发生），优先保证：
  - 值语义正确性（`raw_json`）
  - 再降级定位信息（span 可空，但 json_path 必须可用）

选择理由：

1. 保持公共 API 稳定，不被外部 `miette` 版本影响。
2. 同时满足：
  - 结构化消费（CI、Web UI）
  - 人类可读输出（终端、日志）
3. 避免 B2 的模型侵入与架构耦合。
