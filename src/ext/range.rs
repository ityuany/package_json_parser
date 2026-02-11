use std::ops::Range;

use jsonc_parser::{ast::ObjectProp, common::Ranged};

/// 从 ObjectProp 中按路径提取值的 range。
///
/// - 空路径 `&[]`：返回 prop 直接值的 range
/// - 单层 `&["url"]`：返回嵌套对象中指定字段值的 range
/// - 多层 `&["a", "b"]`：支持任意深度嵌套
pub fn value_range(prop: Option<&ObjectProp>, path: &[&str]) -> Option<Range<usize>> {
  let prop = prop?;

  if path.is_empty() {
    let r = prop.value.range();
    return Some(r.start..r.end);
  }

  let mut obj = prop.value.as_object()?;
  for (i, key) in path.iter().enumerate() {
    let inner = obj.get(key)?;
    if i == path.len() - 1 {
      let r = inner.value.range();
      return Some(r.start..r.end);
    }
    obj = inner.value.as_object()?;
  }

  None
}
