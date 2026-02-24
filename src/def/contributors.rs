use crate::def::Person;
use jsonc_parser::ast::ObjectProp;
use serde::de::{SeqAccess, Visitor, value::SeqAccessDeserializer};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::ext::Validator;

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub struct Contributors(pub Vec<Person>);

impl<'de> Deserialize<'de> for Contributors {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ContributorsVisitor;

    impl<'de> Visitor<'de> for ContributorsVisitor {
      type Value = Contributors;

      fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an array of contributors")
      }

      fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
      where
        S: SeqAccess<'de>,
      {
        let value = Vec::<Person>::deserialize(SeqAccessDeserializer::new(seq))?;
        Ok(Contributors(value))
      }
    }

    deserializer.deserialize_any(ContributorsVisitor)
  }
}

impl Validator for Contributors {
  fn validate(&self, _prop: Option<&ObjectProp>) -> miette::Result<()> {
    // todo!();
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::PackageJsonParser;

  #[test]
  fn should_deserialize_contributors_successfully() {
    let parsed = PackageJsonParser::parse_str(r#"{"contributors":["alice"]}"#);
    assert!(parsed.is_ok());
  }

  #[test]
  fn should_fail_deserialize_contributors_when_type_is_invalid() {
    let parsed = PackageJsonParser::parse_str(r#"{"contributors":{"name":"alice"}}"#);
    assert!(parsed.is_err());
  }

  #[test]
  fn should_fail_deserialize_contributors_when_json_is_invalid() {
    let parsed = PackageJsonParser::parse_str("{");
    assert!(parsed.is_err());
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
