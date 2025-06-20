use crate::def::Person;
use jsonc_parser::ast::ObjectProp;
use miette::MietteDiagnostic;
use serde::{Deserialize, Serialize};

use crate::ext::Validator;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Contributors(pub Vec<Person>);

impl Validator for Contributors {
  fn validate(&self, prop: Option<&ObjectProp>) -> Vec<MietteDiagnostic> {
    let mut diagnostics = vec![];

    // if let Some(contributors) = contributors {
    //   for contributor in contributors.elements.iter() {
    //     let x = contributor.as_object();
    //     let person = Person::from(contributor);
    //     diagnostics.extend(person.validate(None));
    //   }
    // } else {
    //   // diagnostics.extend(person.validate(None));
    // }
    // TODO: validate contributors
    // for person in &self.0 {
    //   if let Some(contributors) = contributors {
    //     for contributor in contributors {
    //       diagnostics.extend(person.validate(contributor));
    //     }
    //   } else {
    //     diagnostics.extend(person.validate(None));
    //   }
    // }
    todo!();

    diagnostics
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
