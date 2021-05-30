#[macro_use]
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use log::debug;
use std::io::Read;
use yaserde::de::from_str;
use yaserde::YaDeserialize;

fn init() {
  let _ = env_logger::builder().is_test(true).try_init();
}

macro_rules! convert_and_validate {
  ($content: expr, $struct: tt, $model: expr) => {
    debug!("convert_and_validate @ {}:{}", file!(), line!());
    let loaded: Result<$struct, String> = from_str($content);
    assert_eq!(loaded, Ok($model));
  };
}

#[test]
fn de_basic() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "book")]
  pub struct Book {
    author: String,
    title: String,
  }

  let content =
    "<book><author>Antoine de Saint-Exupéry</author><title>Little prince</title></book>";
  convert_and_validate!(
    content,
    Book,
    Book {
      author: "Antoine de Saint-Exupéry".to_owned(),
      title: "Little prince".to_owned(),
    }
  );

  let content =
    "<book><title>Little prince</title><author>Antoine de Saint-Exupéry</author></book>";
  convert_and_validate!(
    content,
    Book,
    Book {
      author: "Antoine de Saint-Exupéry".to_owned(),
      title: "Little prince".to_owned(),
    }
  );
}

#[test]
fn de_keyword() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "book")]
  pub struct Book {
    #[yaserde(attribute, rename = "ref")]
    pub r#ref: String,
  }

  let content = "<book ref=\"978-1522968122\"></book>";
  convert_and_validate!(
    content,
    Book,
    Book {
      r#ref: "978-1522968122".to_string()
    }
  );
}

#[test]
fn de_dash_param() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "book")]
  pub struct Book {
    #[yaserde(rename = "author-release")]
    author: String,
    title: String,
  }

  let content =
    "<book><author-release>Antoine de Saint-Exupéry</author-release><title>Little prince</title></book>";
  convert_and_validate!(
    content,
    Book,
    Book {
      author: "Antoine de Saint-Exupéry".to_owned(),
      title: "Little prince".to_owned(),
    }
  );

  let content =
    "<book><title>Little prince</title><author-release>Antoine de Saint-Exupéry</author-release></book>";
  convert_and_validate!(
    content,
    Book,
    Book {
      author: "Antoine de Saint-Exupéry".to_owned(),
      title: "Little prince".to_owned(),
    }
  );
}

#[test]
fn de_multiple_segments() {
  init();

  mod other_mod {
    #[derive(YaDeserialize, PartialEq, Debug, Default)]
    pub struct Page {
      pub number: i32,
      pub text: String,
    }
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "book")]
  pub struct Book {
    author: String,
    title: String,
    page: other_mod::Page,
  }

  let content = r#"
      <book>
        <author>Antoine de Saint-Exupéry</author>
        <title>Little prince</title>
        <page>
          <number>40</number>
          <text>The Earth is not just an ordinary planet!</text>
        </page>
      </book>
    "#;

  convert_and_validate!(
    content,
    Book,
    Book {
      author: "Antoine de Saint-Exupéry".to_owned(),
      title: "Little prince".to_owned(),
      page: other_mod::Page {
        number: 40,
        text: "The Earth is not just an ordinary planet!".to_owned(),
      },
    }
  );
}

#[test]
fn de_list_of_items() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "library")]
  pub struct Library {
    books: Vec<String>,
  }

  let content = "<library><books>Little Prince</books><books>Harry Potter</books></library>";
  convert_and_validate!(
    content,
    Library,
    Library {
      books: vec!["Little Prince".to_owned(), "Harry Potter".to_owned()],
    }
  );

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "libraries")]
  pub struct Libraries {
    library: Vec<Library>,
  }

  let content = "<libraries><library><books>Little Prince</books></library><library><books>Harry Potter</books></library></libraries>";
  convert_and_validate!(
    content,
    Libraries,
    Libraries {
      library: vec![
        Library {
          books: vec!["Little Prince".to_owned()],
        },
        Library {
          books: vec!["Harry Potter".to_owned()],
        },
      ],
    }
  );
}

#[test]
fn de_attributes() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute)]
    item: String,
    sub: SubStruct,
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "sub")]
  pub struct SubStruct {
    #[yaserde(attribute)]
    subitem: String,
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct {
        subitem: "".to_string(),
      }
    }
  }

  let content = "<base item=\"something\"><sub subitem=\"sub-something\"></sub></base>";
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      item: "something".to_string(),
      sub: SubStruct {
        subitem: "sub-something".to_string(),
      },
    }
  );
}

#[test]
fn de_attributes_custom_deserializer() {
  init();

  mod other_mod {
    use super::*;

    use xml::reader::XmlEvent;

    #[derive(Debug, Default, PartialEq)]
    pub struct Attributes {
      pub items: Vec<String>,
    }

    impl YaDeserialize for Attributes {
      fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        loop {
          match reader.next_event()? {
            XmlEvent::StartElement { .. } => {}
            XmlEvent::Characters(ref text_content) => {
              let items: Vec<String> = text_content
                .split(' ')
                .map(|item| item.to_owned())
                .collect();
              return Ok(Attributes { items });
            }
            _ => {
              break;
            }
          }
        }

        Err("Unable to parse attribute".to_string())
      }
    }
  }

  #[derive(Default, YaDeserialize, PartialEq, Debug)]
  pub struct Struct {
    #[yaserde(attribute)]
    attr_option_string: Option<String>,
    #[yaserde(attribute)]
    attr_option_struct: Option<other_mod::Attributes>,
  }

  convert_and_validate!(
    r#"<Struct />"#,
    Struct,
    Struct {
      attr_option_string: None,
      attr_option_struct: None
    }
  );

  convert_and_validate!(
    r#"<Struct attr_option_string="some value" attr_option_struct="variant2 variant3" />"#,
    Struct,
    Struct {
      attr_option_string: Some("some value".to_string()),
      attr_option_struct: Some(other_mod::Attributes {
        items: vec!["variant2".to_string(), "variant3".to_string()]
      })
    }
  );
}

#[test]
fn de_attributes_complex() {
  init();

  mod other_mod {
    #[derive(YaDeserialize, PartialEq, Debug)]
    pub enum AttrEnum {
      #[yaserde(rename = "variant 1")]
      Variant1,
      #[yaserde(rename = "variant 2")]
      Variant2,
    }

    impl Default for AttrEnum {
      fn default() -> AttrEnum {
        AttrEnum::Variant1
      }
    }
  }

  #[derive(Default, YaDeserialize, PartialEq, Debug)]
  pub struct Struct {
    #[yaserde(attribute)]
    attr_option_string: Option<String>,
    #[yaserde(attribute)]
    attr_option_enum: Option<other_mod::AttrEnum>,
  }

  convert_and_validate!(
    r#"<Struct />"#,
    Struct,
    Struct {
      attr_option_string: None,
      attr_option_enum: None
    }
  );

  convert_and_validate!(
    r#"<Struct attr_option_string="some value" attr_option_enum="variant 2" />"#,
    Struct,
    Struct {
      attr_option_string: Some("some value".to_string()),
      attr_option_enum: Some(other_mod::AttrEnum::Variant2)
    }
  );
}

#[test]
fn de_rename() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute, rename = "Item")]
    item: String,
    #[yaserde(rename = "sub")]
    sub_struct: SubStruct,
    #[yaserde(rename = "maj.min.bug")]
    with_dots: String,
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "sub")]
  pub struct SubStruct {
    #[yaserde(attribute, rename = "sub_item")]
    subitem: String,
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct {
        subitem: "".to_string(),
      }
    }
  }

  let content = "<base Item=\"something\"><sub sub_item=\"sub_something\"></sub><maj.min.bug>2.0.1</maj.min.bug></base>";
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      item: "something".to_string(),
      sub_struct: SubStruct {
        subitem: "sub_something".to_string(),
      },
      with_dots: "2.0.1".into()
    }
  );
}

#[test]
fn de_text_content_with_attributes() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute, rename = "Item")]
    item: String,
    #[yaserde(rename = "sub")]
    sub_struct: SubStruct,
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "sub")]
  pub struct SubStruct {
    #[yaserde(attribute, rename = "sub_item")]
    subitem: String,
    #[yaserde(text)]
    text: String,
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct {
        subitem: "".to_string(),
        text: "".to_string(),
      }
    }
  }

  let content =
    "<base Item=\"something\"><sub sub_item=\"sub_something\">text_content</sub></base>";
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      item: "something".to_string(),
      sub_struct: SubStruct {
        subitem: "sub_something".to_string(),
        text: "text_content".to_string(),
      },
    }
  );
}

#[test]
fn de_text_attribute_on_optional_string() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    #[yaserde(text)]
    text: Option<String>,
  }

  let model = XmlStruct {
    text: Some("Testing text".to_string()),
  };
  let content = r#"<base>Testing text</base>"#;
  convert_and_validate!(content, XmlStruct, model);

  let model = XmlStruct { text: None };
  let content = r#"<base></base>"#;
  convert_and_validate!(content, XmlStruct, model);
}

#[test]
fn de_enum() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    background: Color,
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct Colors {
    items: Vec<Color>,
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "color")]
  pub enum Color {
    White,
    Black,
  }

  impl Default for Color {
    fn default() -> Color {
      Color::White
    }
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  pub struct RGBColor {
    red: String,
    green: String,
    blue: String,
  }

  impl Default for RGBColor {
    fn default() -> RGBColor {
      RGBColor {
        red: "0".to_string(),
        green: "0".to_string(),
        blue: "0".to_string(),
      }
    }
  }

  let content =
    "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><background>Black</background></base>";
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Black,
    }
  );

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><items>Black</items><items>White</items></base>";
  convert_and_validate!(
    content,
    Colors,
    Colors {
      items: vec![Color::Black, Color::White],
    }
  );
}

#[test]
fn de_attribute_enum() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute)]
    background: Color,
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "color")]
  pub enum Color {
    White,
    Black,
  }

  impl Default for Color {
    fn default() -> Color {
      Color::White
    }
  }

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base background=\"Black\" />";
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Black,
    }
  );
}

#[test]
fn de_complex_enum() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  pub struct XmlStruct {
    background: Color,
  }

  #[derive(YaDeserialize, PartialEq, Debug, Default)]
  pub struct OtherStruct {
    fi: i32,
    se: i32,
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  pub enum Color {
    White,
    Black(String),
    Orange(String),
    Red(i32),
    Green(OtherStruct),
    Yellow(Option<String>),
    Brown(Option<OtherStruct>),
    Blue(Vec<String>),
    Purple(Vec<i32>),
    Magenta(Vec<OtherStruct>),
    #[yaserde(rename = "NotSoCyan")]
    Cyan(Vec<OtherStruct>),
    #[yaserde(rename = "renamed.with.dots")]
    Dotted(u32),
  }

  impl Default for Color {
    fn default() -> Color {
      Color::White
    }
  }

  let content = r#"<?xml version="1.0" encoding="utf-8"?>
    <base>
      <background>
        <Black>text</Black>
      </background>
    </base>
  "#;
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Black("text".to_owned()),
    }
  );

  let content = r#"<?xml version="1.0" encoding="utf-8"?>
    <base>
      <background>
        <Orange>text</Orange>
      </background>
    </base>
  "#;
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Orange("text".to_owned()),
    }
  );

  let content = r#"<?xml version="1.0" encoding="utf-8"?>
    <base>
      <background>
        <Red>56</Red>
      </background>
    </base>
  "#;
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Red(56),
    }
  );

  let content = r#"<?xml version="1.0" encoding="utf-8"?>
    <base>
      <background>
        <Green>
          <fi>12</fi>
          <se>23</se>
        </Green>
      </background>
    </base>
  "#;
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Green(OtherStruct { fi: 12, se: 23 }),
    }
  );

  let content = r#"<?xml version="1.0" encoding="utf-8"?>
    <base>
      <background>
        <Yellow>text</Yellow>
      </background>
    </base>
  "#;
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Yellow(Some("text".to_owned())),
    }
  );

  let content = r#"<?xml version="1.0" encoding="utf-8"?>
    <base>
      <background>
        <Brown>
          <fi>12</fi>
          <se>23</se>
        </Brown>
      </background>
    </base>
  "#;
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Brown(Some(OtherStruct { fi: 12, se: 23 })),
    }
  );

  let content = r#"<?xml version="1.0" encoding="utf-8"?>
    <base>
      <background>
        <Blue>abc</Blue>
        <Blue>def</Blue>
      </background>
    </base>
  "#;
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Blue(vec!["abc".to_owned(), "def".to_owned()]),
    }
  );

  let content = r#"<?xml version="1.0" encoding="utf-8"?>
    <base>
      <background>
        <Purple>12</Purple>
        <Purple>43</Purple>
      </background>
    </base>
  "#;
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Purple(vec![12, 43]),
    }
  );

  let content = r#"<?xml version="1.0" encoding="utf-8"?>
    <base>
      <background>
        <Magenta><fi>12</fi><se>23</se></Magenta>
        <Magenta><fi>63</fi><se>98</se></Magenta>
      </background>
    </base>
  "#;
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Magenta(vec![
        OtherStruct { fi: 12, se: 23 },
        OtherStruct { fi: 63, se: 98 }
      ]),
    }
  );

  let content = r#"<?xml version="1.0" encoding="utf-8"?>
    <base xmlns:ns="http://www.sample.com/ns/domain">
      <background>
        <NotSoCyan><fi>12</fi><se>23</se></NotSoCyan>
        <NotSoCyan><fi>63</fi><se>98</se></NotSoCyan>
      </background>
    </base>
  "#;
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Cyan(vec![
        OtherStruct { fi: 12, se: 23 },
        OtherStruct { fi: 63, se: 98 }
      ])
    }
  );

  let content = r#"<?xml version="1.0" encoding="utf-8"?>
    <base xmlns:ns="http://www.sample.com/ns/domain">
      <background>
        <renamed.with.dots>54</renamed.with.dots>
      </background>
    </base>
  "#;
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Dotted(54)
    }
  );
}

#[test]
fn de_name_issue_21() {
  init();

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "book")]
  pub struct Book {
    name: String,
  }

  let content = "<book><name>Little prince</name></book>";
  convert_and_validate!(
    content,
    Book,
    Book {
      name: "Little prince".to_owned(),
    }
  );
}

#[test]
fn de_custom() {
  init();

  #[derive(Default, PartialEq, Debug, YaDeserialize)]
  struct Date {
    #[yaserde(rename = "Year")]
    year: i32,
    #[yaserde(rename = "Month")]
    month: i32,
    #[yaserde(rename = "Day")]
    day: Day,
  }

  #[derive(Default, PartialEq, Debug)]
  struct Day {
    value: i32,
  }

  impl YaDeserialize for Day {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
      use std::str::FromStr;

      if let xml::reader::XmlEvent::StartElement { name, .. } = reader.peek()?.to_owned() {
        let expected_name = "Day".to_owned();
        if name.local_name != expected_name {
          return Err(format!(
            "Wrong StartElement name: {}, expected: {}",
            name, expected_name
          ));
        }
        let _next = reader.next_event();
      } else {
        return Err("StartElement missing".to_string());
      }

      if let xml::reader::XmlEvent::Characters(text) = reader.peek()?.to_owned() {
        Ok(Day {
          value: 2 * i32::from_str(&text).unwrap(),
        })
      } else {
        Err("Characters missing".to_string())
      }
    }
  }

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><Date><Year>2020</Year><Month>01</Month><Day>11</Day></Date>";
  let model: Date = from_str(content).unwrap();

  assert_eq!(
    model,
    Date {
      year: 2020,
      month: 1,
      day: Day { value: 11 * 2 }
    }
  );
}

#[test]
fn de_subitem_issue_12() {
  init();

  #[derive(Default, PartialEq, Debug, YaDeserialize)]
  pub struct Struct {
    id: i32,
  }

  convert_and_validate!(
    r#"
    <?xml version="1.0" encoding="utf-8"?>
    <Struct>
      <id>54</id>
      <SubStruct>
        <id>86</id>
      </SubStruct>
    </Struct>
    "#,
    Struct,
    Struct { id: 54 }
  );
}

#[test]
fn de_subitem_issue_12_with_sub() {
  init();

  #[derive(Default, PartialEq, Debug, YaDeserialize)]
  pub struct SubStruct {
    id: i32,
  }

  #[derive(Default, PartialEq, Debug, YaDeserialize)]
  pub struct Struct {
    id: i32,
    #[yaserde(rename = "SubStruct")]
    sub: SubStruct,
  }

  convert_and_validate!(
    r#"
    <?xml version="1.0" encoding="utf-8"?>
    <Struct>
      <id>54</id>
      <SubStruct>
        <id>86</id>
      </SubStruct>
    </Struct>
    "#,
    Struct,
    Struct {
      id: 54,
      sub: SubStruct { id: 86 }
    }
  );
}

#[test]
fn de_subitem_issue_12_attributes() {
  init();

  #[derive(Default, PartialEq, Debug, YaDeserialize)]
  pub struct Struct {
    #[yaserde(attribute)]
    id: i32,
  }

  convert_and_validate!(
    r#"
    <?xml version="1.0" encoding="utf-8"?>
    <Struct id="54">
      <SubStruct id="86" />
    </Struct>
    "#,
    Struct,
    Struct { id: 54 }
  );
}

#[test]
fn de_subitem_issue_12_attributes_with_sub() {
  init();

  #[derive(Default, PartialEq, Debug, YaDeserialize)]
  pub struct SubStruct {
    #[yaserde(attribute)]
    id: i32,
  }

  #[derive(Default, PartialEq, Debug, YaDeserialize)]
  pub struct Struct {
    #[yaserde(attribute)]
    id: i32,
    sub1: SubStruct,
    sub2: SubStruct,
  }

  convert_and_validate!(
    r#"
    <?xml version="1.0" encoding="utf-8"?>
    <Struct id="54">
      <sub1 id="63" />
      <sub2 id="72" />
    </Struct>
    "#,
    Struct,
    Struct {
      id: 54,
      sub1: SubStruct { id: 63 },
      sub2: SubStruct { id: 72 }
    }
  );
}

#[test]
fn de_same_field_name_sub() {
  init();

  #[derive(Default, PartialEq, Debug, YaDeserialize)]
  pub struct SubStruct {
    sub: Option<i32>,
  }

  #[derive(Default, PartialEq, Debug, YaDeserialize)]
  pub struct Struct {
    sub: SubStruct,
  }

  convert_and_validate!("<Struct><sub /></Struct>", Struct, Struct::default());

  convert_and_validate!(
    "<Struct><sub><sub>42</sub></sub></Struct>",
    Struct,
    Struct {
      sub: SubStruct { sub: Some(42) }
    }
  );
}

#[test]
fn de_same_field_name_sub_sub() {
  init();

  #[derive(Default, PartialEq, Debug, YaDeserialize)]
  pub struct SubSubStruct {
    sub: i32,
  }

  #[derive(Default, PartialEq, Debug, YaDeserialize)]
  pub struct SubStruct {
    sub: Option<SubSubStruct>,
  }

  #[derive(Default, PartialEq, Debug, YaDeserialize)]
  pub struct Struct {
    sub: SubStruct,
  }

  convert_and_validate!("<Struct><sub /></Struct>", Struct, Struct::default());

  convert_and_validate!(
    "<Struct>
      <sub>
        <sub>
          <sub>
            42
          </sub>
        </sub>
      </sub>
    </Struct>",
    Struct,
    Struct {
      sub: SubStruct {
        sub: Some(SubSubStruct { sub: 42 })
      }
    }
  );
}

#[test]
fn de_same_field_name_but_some_other_fields_or_something() {
  init();

  #[derive(Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "foo")]
  pub struct FooOuter {
    pub other: bool,
    #[yaserde(rename = "foo")]
    pub foo: Option<FooInner>,
  }

  #[derive(Default, PartialEq, Debug, YaDeserialize, YaSerialize)]
  pub struct FooInner {
    pub blah: bool,
  }

  let content = r#"
    <foo>
      <other>false</other>
      <foo>
        <blah>false</blah>
      </foo>
    </foo>
  "#;

  let model = FooOuter {
    other: false,
    foo: Some(FooInner { blah: false }),
  };

  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, FooOuter);
}
