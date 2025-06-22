use crate::def::Person;
use jsonc_parser::ast::ObjectProp;
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Contributors(pub Vec<Person>);

impl Validator for Contributors {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    // todo!();
    Ok(())
  }
}

// #[cfg(test)]
// mod tests {
//   use crate::case::t;

//   #[test]
//   fn should_pass_validate_contributors() {
//     let jsones = [r#"{"contributors": [{"name": "test"}]}"#];
//     t(&jsones, |parser, parse_result| {
//       let contributors = parse_result
//         .value
//         .as_ref()
//         .and_then(|v| v.as_object())
//         .and_then(|v| v.get("contributors"));

//       if let Some(contributors) = contributors {
//         let res = parser.contributors.unwrap().validate(Some(contributors));
//         assert!(res.is_empty());
//         res
//       } else {
//         let res = parser.contributors.unwrap().validate(None);
//         assert!(!res.is_empty());
//         res
//       }
//     });
//   }

//   #[test]
//   fn should_fail_validate_contributors() {
//     let jsones = [r#"{"contributors": [{"name": ""}]}"#];
//     t(&jsones, |parser, parse_result| {
//       let contributors = parse_result
//         .value
//         .as_ref()
//         .and_then(|v| v.as_object())
//         .and_then(|v| v.get("contributors"));

//       if let Some(contributors) = contributors {
//         let res = parser.contributors.unwrap().validate(Some(contributors));
//         assert!(!res.is_empty());
//         res
//       } else {
//         let res = parser.contributors.unwrap().validate(None);
//         assert!(!res.is_empty());
//         res
//       }
//     });
//   }
// }
