use jsonc_parser::{CollectOptions, ParseOptions, ParseResult, parse_to_ast};
use miette::Result;

pub fn case<'a, S: AsRef<str> + ?Sized>(source: &'a S) -> Result<ParseResult<'a>> {
  let parse_result = parse_to_ast(
    source.as_ref(),
    &CollectOptions::default(),
    &ParseOptions::default(),
  )
  .unwrap();
  Ok(parse_result)
}
