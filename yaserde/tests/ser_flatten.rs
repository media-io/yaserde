#[macro_use]
extern crate yaserde_derive;

use std::io::Write;
use yaserde::ser::to_string;
use yaserde::YaSerialize;

macro_rules! convert_and_validate {
  ($model: expr, $content: expr) => {
    let data: Result<String, String> = to_string(&$model);
    assert_eq!(
      data,
      Ok(
        String::from($content)
          .split("\n")
          .map(|s| s.trim())
          .collect::<String>()
      )
    );
  };
}

#[test]
fn ser_root_flatten_struct() {

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(flatten)]
  pub struct Content {
    binary_data: String,
    string_data: String,
  }

  let model = Content{
    binary_data: "binary".to_string(),
    string_data: "string".to_string(),
  };
  let content = r#"<?xml version="1.0" encoding="utf-8"?><binary_data>binary</binary_data><string_data>string</string_data>"#;
  convert_and_validate!(model, content);
}

#[test]
fn ser_root_flatten_enum() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(flatten)]
  pub enum Content {
    Binary(Binary),
    Data(Data),
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  pub struct Binary {
    binary_data: String,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  pub struct Data {
    string_data: String,
  }

  let model = Content::Binary(Binary{binary_data: "binary".to_string()});
  let content = r#"<?xml version="1.0" encoding="utf-8"?><Binary><binary_data>binary</binary_data></Binary>"#;
  convert_and_validate!(model, content);


  let model = Content::Data(Data{string_data: "string".to_string()});
  let content = r#"<?xml version="1.0" encoding="utf-8"?><Data><string_data>string</string_data></Data>"#;
  convert_and_validate!(model, content);
}
