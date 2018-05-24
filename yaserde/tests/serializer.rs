#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::io::Write;
use yaserde::YaSerialize;
use yaserde::ser::to_string;

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

  assert_eq!(SubStruct::default(), SubStruct{subitem: "".to_string()});

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

  assert_eq!(SubStruct::default(), SubStruct{subitem: "".to_string()});

  let model = XmlStruct {
    item: "something".to_string(),
    sub_struct: SubStruct {
      subitem: "sub_something".to_string(),
    },
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base Item=\"something\"><sub sub_item=\"sub_something\" /></base>";
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

  assert_eq!(SubStruct::default(), SubStruct{
    subitem: "".to_string(),
    text: "".to_string(),
  });

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

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><color><custom><enabled>true</enabled><color><red>0</red><green>128</green><blue>255</blue></color><alpha>Opaque</alpha><alphas>Opaque</alphas><alphas>Transparent</alphas></custom></color></base>";
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
