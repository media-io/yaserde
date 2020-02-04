extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::io::Write;
use yaserde::ser::to_string;
use yaserde::YaSerialize;

macro_rules! convert_and_validate {
  ($model: expr, $content: expr) => {
    let data: Result<String, String> = to_string(&$model);
    assert_eq!(data, Ok(String::from($content)));
  };
}

#[test]
fn ser_basic() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    item: String,
  }

  let model = XmlStruct {
    item: "something".to_string(),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><item>something</item></base>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_list_of_items() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    items: Vec<String>,
  }

  let model = XmlStruct {
    items: vec!["something1".to_string(), "something2".to_string()],
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><items>something1</items><items>something2</items></base>";
  convert_and_validate!(model, content);

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStructOfStruct {
    items: Vec<SubStruct>,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "items")]
  pub struct SubStruct {
    field: String,
  }

  let model2 = XmlStructOfStruct {
    items: vec![
      SubStruct {
        field: "something1".to_string(),
      },
      SubStruct {
        field: "something2".to_string(),
      },
    ],
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><items><field>something1</field></items><items><field>something2</field></items></base>";
  convert_and_validate!(model2, content);
}

#[test]
fn se_attributes() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute)]
    item: String,
    sub: SubStruct,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
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

  assert_eq!(
    SubStruct::default(),
    SubStruct {
      subitem: "".to_string()
    }
  );

  let model = XmlStruct {
    item: "something".to_string(),
    sub: SubStruct {
      subitem: "sub-something".to_string(),
    },
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base item=\"something\"><sub subitem=\"sub-something\" /></base>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_rename() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute, rename = "Item")]
    item: String,
    #[yaserde(rename = "sub")]
    sub_struct: SubStruct,
    #[yaserde(rename = "maj.min.bug")]
    version: String,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
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

  assert_eq!(
    SubStruct::default(),
    SubStruct {
      subitem: "".to_string()
    }
  );

  let model = XmlStruct {
    item: "something".to_string(),
    sub_struct: SubStruct {
      subitem: "sub_something".to_string(),
    },
    version: "2.0.2".into(),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base Item=\"something\"><sub sub_item=\"sub_something\" /><maj.min.bug>2.0.2</maj.min.bug></base>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_text_content_with_attributes() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute, rename = "Item")]
    item: String,
    #[yaserde(rename = "sub")]
    sub_struct: SubStruct,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
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

  assert_eq!(
    SubStruct::default(),
    SubStruct {
      subitem: "".to_string(),
      text: "".to_string(),
    }
  );

  let model = XmlStruct {
    item: "something".to_string(),
    sub_struct: SubStruct {
      subitem: "sub_something".to_string(),
      text: "text_content".to_string(),
    },
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base Item=\"something\"><sub sub_item=\"sub_something\">text_content</sub></base>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_enum() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    color: Color,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "color")]
  pub enum Color {
    White,
    Black,
    #[yaserde(rename = "custom")]
    Custom {
      enabled: String,
      color: RGBColor,
      alpha: Alpha,
      alphas: Vec<Alpha>,
    },
  }

  impl Default for Color {
    fn default() -> Color {
      Color::White
    }
  }

  assert_eq!(Color::default(), Color::White);

  #[derive(YaSerialize, PartialEq, Debug)]
  pub struct RGBColor {
    red: String,
    green: String,
    blue: String,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  pub enum Alpha {
    Transparent,
    Opaque,
  }

  let model = XmlStruct {
    color: Color::Black,
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color>Black</color></base>";
  convert_and_validate!(model, content);

  let model = XmlStruct {
    color: Color::Custom {
      enabled: "true".to_string(),
      color: RGBColor {
        red: "0".to_string(),
        green: "128".to_string(),
        blue: "255".to_string(),
      },
      alpha: Alpha::Opaque,
      alphas: vec![Alpha::Opaque, Alpha::Transparent],
    },
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color><enabled>true</enabled><color><red>0</red><green>128</green><blue>255</blue></color><alpha>Opaque</alpha><alphas>Opaque</alphas><alphas>Transparent</alphas></color></base>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_attribute_enum() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute)]
    color: Color,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "color")]
  pub enum Color {
    #[yaserde(rename = "pink")]
    Pink,
  }

  let model = XmlStruct { color: Color::Pink };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base color=\"pink\" />";
  convert_and_validate!(model, content);
}

#[test]
fn ser_unnamed_enum() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    color: Enum,
  }

  #[derive(YaSerialize, PartialEq, Debug, Default)]
  pub struct OtherStruct {
    fi: i32,
    se: i32,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  pub enum Enum {
    Simple,
    Field(String),
    FullPath(std::string::String),
    Integer(i32),
    UserStruct(OtherStruct),
    OptionString(Option<String>),
    OptionUserStruct(Option<OtherStruct>),
    Strings(Vec<String>),
    Ints(Vec<i32>),
    Structs(Vec<OtherStruct>),
    #[yaserde(rename = "renamed")]
    ToRename(u32),
    #[yaserde(rename = "renamed.with.dots")]
    ToRenameDots(u32),
  }

  impl Default for Enum {
    fn default() -> Enum {
      Enum::Simple
    }
  }

  let model = XmlStruct {
    color: Enum::Field(String::from("some_text")),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color><Field>some_text</Field></color></base>";
  convert_and_validate!(model, content);

  let model = XmlStruct {
    color: Enum::FullPath(String::from("some_text")),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color><FullPath>some_text</FullPath></color></base>";
  convert_and_validate!(model, content);

  let model = XmlStruct {
    color: Enum::Integer(56),
  };

  let content =
    "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color><Integer>56</Integer></color></base>";
  convert_and_validate!(model, content);

  let model = XmlStruct {
    color: Enum::UserStruct(OtherStruct { fi: 24, se: 42 }),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color><UserStruct><fi>24</fi><se>42</se></UserStruct></color></base>";
  convert_and_validate!(model, content);

  let model = XmlStruct {
    color: Enum::OptionString(Some(String::from("some_text"))),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color><OptionString>some_text</OptionString></color></base>";
  convert_and_validate!(model, content);

  let model = XmlStruct {
    color: Enum::OptionString(None),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color /></base>";
  convert_and_validate!(model, content);

  let model = XmlStruct {
    color: Enum::OptionUserStruct(Some(OtherStruct { fi: 12, se: 23 })),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color><OptionUserStruct><fi>12</fi><se>23</se></OptionUserStruct></color></base>";
  convert_and_validate!(model, content);

  let model = XmlStruct {
    color: Enum::OptionUserStruct(None),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color /></base>";
  convert_and_validate!(model, content);

  let model = XmlStruct {
    color: Enum::Strings(vec![String::from("abc"), String::from("def")]),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color><Strings>abc</Strings><Strings>def</Strings></color></base>";
  convert_and_validate!(model, content);

  let model = XmlStruct {
    color: Enum::Ints(vec![23, 45]),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color><Ints>23</Ints><Ints>45</Ints></color></base>";
  convert_and_validate!(model, content);

  let model = XmlStruct {
    color: Enum::Structs(vec![
      OtherStruct { fi: 12, se: 23 },
      OtherStruct { fi: 34, se: 45 },
    ]),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color><Structs><fi>12</fi><se>23</se></Structs><Structs><fi>34</fi><se>45</se></Structs></color></base>";
  convert_and_validate!(model, content);

  let model = XmlStruct {
    color: Enum::ToRename(87),
  };

  let content =
    "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color><renamed>87</renamed></color></base>";
  convert_and_validate!(model, content);

  let model = XmlStruct {
    color: Enum::ToRenameDots(84),
  };

  let content =
    "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color><renamed.with.dots>84</renamed.with.dots></color></base>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_name_issue_21() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    name: String,
  }

  let model = XmlStruct {
    name: "something".to_string(),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><name>something</name></base>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_custom() {
  #[derive(Default, PartialEq, Debug, YaSerialize)]
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

  impl YaSerialize for Day {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
      let _ret = writer.write(xml::writer::XmlEvent::start_element("DoubleDay"));
      let _ret = writer.write(xml::writer::XmlEvent::characters(
        &(self.value * 2).to_string(),
      ));
      let _ret = writer.write(xml::writer::XmlEvent::end_element());
      Ok(())
    }
  }

  let model = Date {
    year: 2020,
    month: 1,
    day: Day { value: 5 },
  };
  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><Date><Year>2020</Year><Month>1</Month><DoubleDay>10</DoubleDay></Date>";
  convert_and_validate!(model, content);
}
