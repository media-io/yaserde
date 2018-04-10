
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;
extern crate xml;

use std::str;
use std::io::Cursor;
use std::io::Write;
use xml::writer::EventWriter;
use yaserde::YaSerialize;

macro_rules! convert_and_validate {
  ($model:expr, $content:expr) => (
    let mut buf = Cursor::new(Vec::new());
    let mut writer = EventWriter::new(&mut buf);
    let _status = $model.derive_serialize(&mut writer);

    let buffer = writer.into_inner();
    let cursor = buffer.get_ref();

    let data = str::from_utf8(cursor).expect("Found invalid UTF-8");
    assert_eq!(data, $content);
  )
}

#[test]
fn ser_basic() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    item: String
  }

  let model = XmlStruct {
    item: "something".to_string()
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><item>something</item></base>".to_string();
  convert_and_validate!(model, content);
}

#[test]
fn ser_list_of_items() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    items: Vec<String>
  }

  let model = XmlStruct{
    items: vec![
      "something1".to_string(),
      "something2".to_string()
    ]
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><items>something1</items><items>something2</items></base>".to_string();
  convert_and_validate!(model, content);
}

#[test]
fn se_attributes() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    #[yaserde(attribute)]
    item: String,
    sub: SubStruct
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root="sub")]
  pub struct SubStruct {
    #[yaserde(attribute)]
    subitem: String
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct{
        subitem: "".to_string()
      }
    }
  }

  let model = XmlStruct{
    item: "something".to_string(),
    sub: SubStruct{
      subitem: "sub-something".to_string()
    }
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base item=\"something\"><sub subitem=\"sub-something\" /></base>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_rename() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    #[yaserde(attribute, rename="Item")]
    item: String,
    #[yaserde(rename="sub")]
    sub_struct: SubStruct
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root="sub")]
  pub struct SubStruct {
    #[yaserde(attribute, rename="sub_item")]
    subitem: String,
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct{
        subitem: "".to_string()
      }
    }
  }

  let model = XmlStruct{
    item: "something".to_string(),
    sub_struct: SubStruct{
      subitem: "sub_something".to_string()
    }
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base Item=\"something\"><sub sub_item=\"sub_something\" /></base>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_text_content_with_attributes() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    #[yaserde(attribute, rename="Item")]
    item: String,
    #[yaserde(rename="sub")]
    sub_struct: SubStruct
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root="sub")]
  pub struct SubStruct {
    #[yaserde(attribute, rename="sub_item")]
    subitem: String,
    #[yaserde(text)]
    text: String,
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct{
        subitem: "".to_string(),
        text: "".to_string(),
      }
    }
  }

  let model = XmlStruct{
    item: "something".to_string(),
    sub_struct: SubStruct{
      subitem: "sub_something".to_string(),
      text: "text_content".to_string()
    }
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base Item=\"something\"><sub sub_item=\"sub_something\">text_content</sub></base>";
  convert_and_validate!(model, content);
}
