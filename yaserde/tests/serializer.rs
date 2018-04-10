
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;
extern crate xml;

use std::str;
use std::io::Cursor;
use std::io::Write;
use xml::writer::EventWriter;
use yaserde::{YaSerialize};

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

  let mut buf = Cursor::new(Vec::new());
  let mut writer = EventWriter::new(&mut buf);
  let _status = model.derive_serialize(&mut writer, None);

  let buffer = writer.into_inner();
  let cursor = buffer.get_ref();

  let data = str::from_utf8(cursor).expect("Found invalid UTF-8");
  assert_eq!(data, content);
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

  let mut buf = Cursor::new(Vec::new());
  let mut writer = EventWriter::new(&mut buf);
  let _status = model.derive_serialize(&mut writer, None);

  let buffer = writer.into_inner();
  let cursor = buffer.get_ref();

  let data = str::from_utf8(cursor).expect("Found invalid UTF-8");
  assert_eq!(data, content);

}