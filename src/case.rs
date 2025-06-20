use jsonc_parser::{CollectOptions, ParseOptions, ParseResult, parse_to_ast};
use miette::{MietteDiagnostic, Result};

use crate::PackageJsonParser;

pub fn case<'a, S: AsRef<str> + ?Sized>(source: &'a S) -> Result<ParseResult<'a>> {
  let parse_result = parse_to_ast(
    source.as_ref(),
    &CollectOptions::default(),
    &ParseOptions::default(),
  )
  .unwrap();
  Ok(parse_result)
}

pub fn t<F>(jsones: &[&'static str], callback: F)
where
  F: Fn(PackageJsonParser, ParseResult) -> Vec<MietteDiagnostic>,
{
  for json in jsones {
    let parser = PackageJsonParser::parse_str(json).unwrap();
    let parse_result = parse_to_ast(
      json.as_ref(),
      &CollectOptions::default(),
      &ParseOptions::default(),
    )
    .unwrap();
    let res = callback(parser.clone(), parse_result);
    for res in res {
      let report = miette::Report::from(res).with_source_code(parser.__raw_source.clone().unwrap());
      println!("{:?}", report);
    }
  }
}
