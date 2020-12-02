#[macro_use]
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

#[test]
fn basic_enum() {
  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    color: Color,
  }

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "color")]
  pub enum Color {
    White,
    Black,
    #[yaserde(rename = "custom")]
    Custom {
      enabled: String,
      u8_value: u8,
      i8_value: i8,
      u16_value: u16,
      i16_value: i16,
      u32_value: u32,
      i32_value: i32,
      u64_value: u64,
      i64_value: i64,
      f32_value: f32,
      f64_value: f64,
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

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  pub struct RGBColor {
    red: String,
    green: String,
    blue: String,
  }

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  pub enum Alpha {
    Transparent,
    Opaque,
  }

  impl Default for Alpha {
    fn default() -> Alpha {
      Alpha::Transparent
    }
  }

  let model = XmlStruct {
    color: Color::Black,
  };

  let content = "<base><color>Black</color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Color::Custom {
      enabled: "true".to_string(),
      u8_value: 8,
      i8_value: -8,
      u16_value: 16,
      i16_value: -16,
      u32_value: 32,
      i32_value: -32,
      u64_value: 64,
      i64_value: -64,
      f32_value: 32.0,
      f64_value: 64.0,
      color: RGBColor {
        red: "0".to_string(),
        green: "128".to_string(),
        blue: "255".to_string(),
      },
      alpha: Alpha::Opaque,
      alphas: vec![Alpha::Opaque, Alpha::Transparent],
    },
  };

  let content = r#"
<base>
<color><enabled>true</enabled>
<u8_value>8</u8_value>
<i8_value>-8</i8_value>
<u16_value>16</u16_value>
<i16_value>-16</i16_value>
<u32_value>32</u32_value>
<i32_value>-32</i32_value>
<u64_value>64</u64_value>
<i64_value>-64</i64_value>
<f32_value>32</f32_value>
<f64_value>64</f64_value>
<color><red>0</red><green>128</green><blue>255</blue></color>
<alpha>Opaque</alpha>
<alphas>Opaque</alphas>
<alphas>Transparent</alphas>
</color>
</base>"#;

  serialize_and_validate!(model, content);
  // TODO
  // deserialize_and_validate!(content, model, XmlStruct);
}

#[test]
fn attribute_enum() {
  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute)]
    color: Color,
  }

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "color")]
  pub enum Color {
    #[yaserde(rename = "pink")]
    Pink,
  }

  impl Default for Color {
    fn default() -> Color {
      Color::Pink
    }
  }

  let model = XmlStruct { color: Color::Pink };

  let content = r#"<base color="pink" />"#;
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);
}

#[test]
fn unnamed_enum() {
  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    color: Enum,
  }

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  pub struct OtherStruct {
    fi: i32,
    se: i32,
  }

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  pub enum Enum {
    Simple,
    Field(String),
    FullPath(String),
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
    color: Enum::Field("some_text".to_owned()),
  };

  let content = "<base><color><Field>some_text</Field></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::FullPath("some_text".to_owned()),
  };

  let content = "<base><color><FullPath>some_text</FullPath></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::Integer(56),
  };

  let content = "<base><color><Integer>56</Integer></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::UserStruct(OtherStruct { fi: 24, se: 42 }),
  };

  let content = "<base><color><UserStruct><fi>24</fi><se>42</se></UserStruct></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::OptionString(Some("some_text".to_owned())),
  };

  let content = "<base><color><OptionString>some_text</OptionString></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::OptionString(None),
  };

  let content = "<base><color /></base>";
  serialize_and_validate!(model, content);
  // TODO
  // deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::OptionUserStruct(Some(OtherStruct { fi: 12, se: 23 })),
  };

  let content =
    "<base><color><OptionUserStruct><fi>12</fi><se>23</se></OptionUserStruct></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::OptionUserStruct(None),
  };

  let content = "<base><color /></base>";
  serialize_and_validate!(model, content);
  // TODO
  // deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::Strings(vec!["abc".to_owned(), "def".to_owned()]),
  };

  let content = "<base><color><Strings>abc</Strings><Strings>def</Strings></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::Ints(vec![23, 45]),
  };

  let content = "<base><color><Ints>23</Ints><Ints>45</Ints></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::Structs(vec![
      OtherStruct { fi: 12, se: 23 },
      OtherStruct { fi: 34, se: 45 },
    ]),
  };

  let content = "<base><color><Structs><fi>12</fi><se>23</se></Structs><Structs><fi>34</fi><se>45</se></Structs></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::ToRename(87),
  };

  let content = "<base><color><renamed>87</renamed></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let model = XmlStruct {
    color: Enum::ToRenameDots(84),
  };

  let content = "<base><color><renamed.with.dots>84</renamed.with.dots></color></base>";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);
}
