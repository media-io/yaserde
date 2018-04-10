
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;
extern crate xml;

use std::io::Read;
use xml::reader::EventReader;
use yaserde::{YaDeserialize};

#[test]
fn de_basic() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    item: String
  }

  let content = "<base><item>something</item></base>".to_string();
  let mut parser = EventReader::from_str(content.as_str());

  let loaded = XmlStruct::derive_deserialize(&mut parser, None);
  assert_eq!(loaded, Ok(XmlStruct{
    item: "something".to_string()
  }));
}

#[test]
fn de_list_of_items() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    items: Vec<String>
  }

  let content = "<base><items>something1</items><items>something2</items></base>".to_string();
  let mut parser = EventReader::from_str(content.as_str());

  let loaded = XmlStruct::derive_deserialize(&mut parser, None);
  assert_eq!(loaded, Ok(XmlStruct{
    items: vec![
      "something1".to_string(),
      "something2".to_string()
    ]
  }));
}

#[test]
fn de_attributes() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    #[yaserde(attribute)]
    item: String,
    sub: SubStruct
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
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

  let content = "<base item=\"something\"><sub subitem=\"sub-something\"></sub></base>".to_string();
  let mut parser = EventReader::from_str(content.as_str());

  let loaded = XmlStruct::derive_deserialize(&mut parser, None);
  assert_eq!(loaded, Ok(XmlStruct{
    item: "something".to_string(),
    sub: SubStruct{
      subitem: "sub-something".to_string()
    }
  }));
}

#[test]
fn de_rename() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    #[yaserde(attribute, rename="Item")]
    item: String,
    #[yaserde(rename="sub")]
    sub_struct: SubStruct
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
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

  let content = "<base Item=\"something\"><sub sub_item=\"sub_something\"></sub></base>".to_string();
  let mut parser = EventReader::from_str(content.as_str());

  let loaded = XmlStruct::derive_deserialize(&mut parser, None);
  assert_eq!(loaded, Ok(XmlStruct{
    item: "something".to_string(),
    sub_struct: SubStruct{
      subitem: "sub_something".to_string()
    }
  }));
}

#[test]
fn de_text_content_with_attributes() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="base")]
  pub struct XmlStruct {
    #[yaserde(attribute, rename="Item")]
    item: String,
    #[yaserde(rename="sub")]
    sub_struct: SubStruct
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="sub")]
  pub struct SubStruct {
    #[yaserde(attribute, rename="sub_item")]
    subitem: String,
    #[yaserde(text)]
    text: String
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct{
        subitem: "".to_string(),
        text: "".to_string(),
      }
    }
  }

  let content = "<base Item=\"something\"><sub sub_item=\"sub_something\">text_content</sub></base>".to_string();
  let mut parser = EventReader::from_str(content.as_str());

  let loaded = XmlStruct::derive_deserialize(&mut parser, None);
  assert_eq!(loaded, Ok(XmlStruct{
    item: "something".to_string(),
    sub_struct: SubStruct{
      subitem: "sub_something".to_string(),
      text: "text_content".to_string()
    }
  }));
}
