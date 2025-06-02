use jsonc_parser::common::Ranged;
use miette::{Diagnostic, NamedSource, Result, SourceSpan};
use serde::de::Error;
use serde_valid::validation::Errors as ValidationErrors;
use std::{
  fmt,
  fs::{self},
  path::Path,
};

#[derive(Debug)]
pub struct ValidationErrorWrapper(pub ValidationErrors);

impl fmt::Display for ValidationErrorWrapper {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // for (field, errors) in self.0.iter() {
    //   writeln!(f, "Field '{}': {:?}", field, errors)?;
    // }
    Ok(())
  }
}

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum ErrorKind {
  #[error("name is required")]
  #[diagnostic(code(package_json::name_required))]
  NameRequired,

  #[error("validation failed: {errors}")]
  #[diagnostic(code(package_json::validation_failed))]
  ValidationFailed {
    #[source_code]
    src: String,
    #[label("validation errors occurred here")]
    span: SourceSpan,
    errors: ValidationErrorWrapper,
  },

  #[error("JSON parsing failed")]
  #[diagnostic(code(package_json::json_parse_failed), url(docsrs))]
  JsonParseError {
    #[source_code]
    src: NamedSource<String>,

    #[label("{label_text}")]
    span: SourceSpan,
    label_text: String,

    #[source]
    source: serde_json::Error,

    #[help]
    advice: Option<String>,
  },

  #[error("IO error")]
  #[diagnostic(code(package_json::io_error))]
  IoError(#[from] std::io::Error),
}

impl ErrorKind {
  pub fn validation_failed(src: String, errors: ValidationErrors) -> Self {
    let span = SourceSpan::from(0..src.len());
    Self::ValidationFailed {
      src,
      span,
      errors: ValidationErrorWrapper(errors),
    }
  }

  // pub fn json_parse_error(src: String, source: serde_json::Error) -> Self {
  //   let span = match (source.line(), source.column()) {
  //     (Some(line), Some(column)) => {
  //       let offset = src
  //         .lines()
  //         .take(line.saturating_sub(1))
  //         .map(|l| l.len() + 1)
  //         .sum::<usize>()
  //         + column;
  //       SourceSpan::from(offset..offset + 1)
  //     }
  //     _ => SourceSpan::from(0..src.len()),
  //   };

  //   Self::JsonParseError { src, span, source }
  // }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PackageJsonParserDemo {
  pub name: String,
  pub version: String,
  pub description: String,
  pub main: String,
  pub private: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub author: Option<package_json_parser::Person>,
}

impl PackageJsonParserDemo {
  pub fn parse<P: AsRef<Path>>(path: P) -> Result<()> {
    let content = fs::read_to_string(path.as_ref())
      .map_err(ErrorKind::IoError)
      .map_err(|e| miette::miette!(e))?;

    let package_json_parser: PackageJsonParserDemo =
      serde_json::from_str(&content).map_err(|e| {
        let line = e.line();
        let column = e.column();
        eprintln!("e: {:?}", e);

        println!("line: {}, column: {}", line, column);

        let lines_before = content.lines().take(line.saturating_sub(1));
        let offset = lines_before.map(|l| l.len() + 1).sum::<usize>();

        let len = content
          .lines()
          .nth(line.saturating_sub(1))
          .unwrap_or("")
          .len();

        println!(
          "offset: {}, len: {}, classify: {}",
          offset,
          len,
          e.to_string(),
        );

        let span = SourceSpan::from(offset..offset + len);

        let name_source = NamedSource::new(path.as_ref().to_str().unwrap(), content.clone());

        let d = ErrorKind::JsonParseError {
          src: name_source,
          span,
          label_text: e.to_string(),
          source: e,
          advice: Some("Please check the JSON syntax".to_string()),
        };

        miette::miette!(d)
      })?;

    println!("package_json_parser: {:?}", package_json_parser);

    // let mut reader = BufReader::new(file);
    // let mut content = String::new();
    // reader.read_to_string(&mut content)?;
    // let mut package_json_parser: PackageJsonParserDemo = serde_json::from_str(&content)?;
    Ok(())
  }
}

fn main() -> Result<()> {
  miette::set_hook(Box::new(|_| {
    Box::new(
      miette::MietteHandlerOpts::new()
        .terminal_links(true)
        .unicode(true)
        .context_lines(10)
        .tab_width(4)
        .break_words(true)
        .build(),
    )
  }))?;

  // PackageJsonParserDemo::parse("/Users/ityuany/GitRepository/csp-new/package.json")?;

  h()?;

  Ok(())
}

fn h() -> Result<()> {
  let content = fs::read_to_string("/Users/ityuany/GitRepository/csp-new/package.json").unwrap();

  let parse_result = jsonc_parser::parse_to_ast(
    &content,
    &jsonc_parser::CollectOptions {
      comments: jsonc_parser::CommentCollectionStrategy::Separate, // include comments in result
      tokens: true,                                                // include tokens in result
    },
    &Default::default(),
  )
  .unwrap();

  // if let Some(tokens) = parse_result.tokens {
  //   for token in tokens {
  //     println!("token: {:?}", token.token);
  //   }
  // }

  let root = parse_result.value.unwrap();

  let root = root.as_object().unwrap();

  for k in root.properties.iter() {
    if k.name.as_str() == "private" {
      println!("key: {} {:?}", k.name.as_str(), k.name.range());

      if let Some(value) = k.value.as_boolean_lit() {
        println!("value: {} {:?}", value.value, value.range());
        let name_source = NamedSource::new(
          "/Users/ityuany/GitRepository/csp-new/package.json",
          content.clone(),
        );

        let range = value.range();

        let offset = range.start;
        let len = range.end - range.start;

        println!("offset: {} , len: {}", offset, len);

        let span = SourceSpan::from(offset..range.end);

        let d = ErrorKind::JsonParseError {
          src: name_source,
          span,
          label_text: "private must be a boolean".to_string(),
          source: serde_json::Error::custom("private must be a boolean"),
          advice: Some("Please check the JSON syntax".to_string()),
        };

        return Err(miette::miette!(d));
      }
    }
  }

  Ok(())
}
