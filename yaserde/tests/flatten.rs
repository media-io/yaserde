#[macro_use]
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

fn init() {
  let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn basic_flatten() {
  init();

  #[derive(Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
  struct DateTime {
    #[yaserde(flatten)]
    date: Date,
    time: String,
    #[yaserde(flatten)]
    kind: DateKind,
  }

  #[derive(Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
  struct Date {
    year: i32,
    month: i32,
    day: i32,
    #[yaserde(flatten)]
    extra: Extra,
    #[yaserde(flatten)]
    optional_extra: Option<OptionalExtra>,
  }

  #[derive(Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
  pub struct Extra {
    week: i32,
    century: i32,
  }

  #[derive(Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
  pub struct OptionalExtra {
    lunar_day: i32,
  }

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
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
  }

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

  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, DateTime);
}

#[test]
fn root_flatten_struct() {
  init();

  #[derive(YaDeserialize, YaSerialize, PartialEq, Debug)]
  #[yaserde(flatten)]
  pub struct Content {
    binary_data: String,
    string_data: String,
  }

  let model = Content {
    binary_data: "binary".to_string(),
    string_data: "string".to_string(),
  };

  let content = "<binary_data>binary</binary_data><string_data>string</string_data>";

  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, Content);
}

#[test]
fn root_flatten_enum() {
  init();

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

  let content = "<Binary><binary_data>binary</binary_data></Binary>";
  serialize_and_validate!(model, content);

  let model = Content::Data(Data {
    string_data: "string".to_string(),
  });

  let content = "<Data><string_data>string</string_data></Data>";
  serialize_and_validate!(model, content);
}

#[test]
fn flatten_attribute() {
  init();

  #[derive(Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
  struct HtmlText {
    #[yaserde(flatten)]
    text_attributes: TextAttributes,
    #[yaserde(attribute)]
    display: String,
  }

  #[derive(Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
  struct TextAttributes {
    #[yaserde(attribute)]
    bold: bool,
    #[yaserde(flatten)]
    font: FontAttributes,
  }

  #[derive(Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
  #[yaserde(namespace = "ns: http://www.sample.com/ns/domain")]
  pub struct FontAttributes {
    #[yaserde(attribute, prefix = "ns")]
    size: u32,
  }

  let model = HtmlText {
    text_attributes: TextAttributes {
      bold: true,
      font: FontAttributes { size: 24 },
    },
    display: "block".to_string(),
  };

  let content = r#"
    <HtmlText xmlns:ns="http://www.sample.com/ns/domain" display="block" bold="true" ns:size="24" />"#;

  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, HtmlText);
}
