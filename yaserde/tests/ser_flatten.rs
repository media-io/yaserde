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
fn ser_flatten() {
  #[derive(Default, PartialEq, Debug, YaSerialize)]
  struct DateTime {
    #[yaserde(flatten)]
    date: Date,
    time: String,
    #[yaserde(flatten)]
    kind: DateKind,
  }

  #[derive(Default, PartialEq, Debug, YaSerialize)]
  struct Date {
    year: i32,
    month: i32,
    day: i32,
    #[yaserde(flatten)]
    extra: Extra,
    #[yaserde(flatten)]
    optional_extra: Option<OptionalExtra>,
  }

  #[derive(Default, PartialEq, Debug, YaSerialize)]
  pub struct Extra {
    week: i32,
    century: i32,
  }

  #[derive(Default, PartialEq, Debug, YaSerialize)]
  pub struct OptionalExtra {
    lunar_day: i32,
  }

  #[derive(PartialEq, Debug, YaSerialize)]
  pub enum DateKind {
    #[yaserde(rename = "holidays")]
    Holidays(Vec<String>),
    #[yaserde(rename = "working")]
    Working,
  }

  impl Default for DateKind {
    fn default() -> Self {
      DateKind::Working
    }
  };

  let model = DateTime {
    date: Date {
      year: 2020,
      month: 1,
      day: 1,
      extra: Extra {
        week: 1,
        century: 21,
      },
      optional_extra: Some(OptionalExtra { lunar_day: 1 }),
    },
    time: "10:40:03".to_string(),
    kind: DateKind::Holidays(vec![
      "New Year's Day".into(),
      "Novy God Day".into(),
      "Polar Bear Swim Day".into(),
    ]),
  };

  let content = r#"
    <?xml version="1.0" encoding="utf-8"?>
    <DateTime>
      <year>2020</year>
      <month>1</month>
      <day>1</day>
      <week>1</week>
      <century>21</century>
      <lunar_day>1</lunar_day>
      <time>10:40:03</time>
      <holidays>New Year's Day</holidays>
      <holidays>Novy God Day</holidays>
      <holidays>Polar Bear Swim Day</holidays>
    </DateTime>"#;

  convert_and_validate!(model, content);
}

#[test]
fn ser_root_flatten_struct() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(flatten)]
  pub struct Content {
    binary_data: String,
    string_data: String,
  }

  let model = Content {
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

  let model = Content::Binary(Binary {
    binary_data: "binary".to_string(),
  });
  let content =
    r#"<?xml version="1.0" encoding="utf-8"?><Binary><binary_data>binary</binary_data></Binary>"#;
  convert_and_validate!(model, content);

  let model = Content::Data(Data {
    string_data: "string".to_string(),
  });
  let content =
    r#"<?xml version="1.0" encoding="utf-8"?><Data><string_data>string</string_data></Data>"#;
  convert_and_validate!(model, content);
}
