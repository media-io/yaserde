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

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
  struct DateTime {
    #[yaserde(flatten = true)]
    date: Date,
    time: String,
    #[yaserde(flatten = true)]
    kind: DateKind,
  }

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
  struct Date {
    year: i32,
    month: i32,
    day: i32,
    #[yaserde(flatten = true)]
    extra: Extra,
    #[yaserde(flatten = true)]
    optional_extra: Option<OptionalExtra>,
  }

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
  pub struct Extra {
    week: i32,
    century: i32,
  }

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
  pub struct OptionalExtra {
    lunar_day: i32,
  }

  #[derive(Debug, Default, PartialEq, YaDeserialize, YaSerialize)]
  pub enum DateKind {
    #[yaserde(rename = "holidays")]
    Holidays(Vec<String>),
    #[yaserde(rename = "working")]
    #[default]
    Working,
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
  #[yaserde(flatten = true)]
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
  #[yaserde(flatten = true)]
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

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
  struct HtmlText {
    #[yaserde(flatten = true)]
    text_attributes: TextAttributes,
    #[yaserde(attribute = true)]
    display: String,
  }

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
  struct TextAttributes {
    #[yaserde(attribute = true)]
    bold: bool,
    #[yaserde(flatten = true)]
    font: FontAttributes,
  }

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
  #[yaserde(namespaces = {
    "ns" = "http://www.sample.com/ns/domain"
  })]
  pub struct FontAttributes {
    #[yaserde(attribute = true, prefix = "ns")]
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

#[test]
fn flatten_attribute_and_child() {
  init();

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
  struct Node {
    #[yaserde(flatten = true)]
    base: Base,
    value: StringValue,
  }

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
  struct Base {
    #[yaserde(attribute = true)]
    id: String,
  }

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
  struct StringValue {
    #[yaserde(text = true)]
    string: String,
  }

  let model = Node {
    base: Base {
      id: "Foo".to_owned(),
    },
    value: StringValue {
      string: "Bar".to_owned(),
    },
  };

  let content = r#"<Node id="Foo"><value>Bar</value></Node>"#;

  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, Node);
}

#[test]
fn flatten_name_in_unknown_child() {
  init();

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
  pub struct Node {
    #[yaserde(flatten = true)]
    base: Base,
    value: Value,
  }

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
  struct Base {
    #[yaserde(attribute = true)]
    id: String,
  }

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
  enum Value {
    Foo(FooStruct),
  }

  impl Default for Value {
    fn default() -> Self {
      Self::Foo(FooStruct {})
    }
  }

  #[derive(PartialEq, Debug, YaDeserialize, YaSerialize)]
  struct FooStruct {}

  let model = Node {
    base: Base {
      id: "Foo".to_owned(),
    },
    value: Value::default(),
  };

  let content = r#"<Node id="Foo"><value><Foo /></value></Node>"#;
  serialize_and_validate!(model, content);

  let content = r#"<Node id="Foo"><value><SomethingThatDoesntExist><value></value></SomethingThatDoesntExist></value></Node>"#;
  deserialize_and_validate!(content, model, Node);
}
