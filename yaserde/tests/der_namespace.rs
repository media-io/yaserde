
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;
extern crate xml;
#[macro_use]
extern crate log;

use std::io::Read;
use yaserde::YaDeserialize;
use yaserde::de::from_str;

macro_rules! convert_and_validate {
  ($content:expr, $struct:tt, $model:expr) => {
    let loaded : Result<$struct, String> = from_str($content);
    assert_eq!(loaded, Ok($model));
  }
}

#[test]
fn de_struct_namespace() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="book", prefix="ns", namespace="ns: http://www.sample.com/ns/domain")]
  pub struct Book {
    #[yaserde(prefix="ns")]
    author: String,
    #[yaserde(prefix="ns")]
    title: String,
  }

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><ns:book xmlns:ns=\"http://www.sample.com/ns/domain\"><ns:author>Antoine de Saint-Exupéry</ns:author><ns:title>Little prince</ns:title></ns:book>";
  convert_and_validate!(content, Book, Book{
    author: String::from("Antoine de Saint-Exupéry"),
    title: String::from("Little prince")
  });

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><ns:book xmlns:ns=\"http://www.sample.com/ns/domain2\"><ns:author>Antoine de Saint-Exupéry</ns:author><ns:title>Little prince</ns:title></ns:book>";
  let loaded : Result<Book, String> = from_str(content);
  assert_eq!(loaded, Err("bad namespace".to_string()));
}

#[test]
fn de_enum_namespace() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root="root", prefix="ns", namespace="ns: http://www.sample.com/ns/domain")]
  pub enum XmlStruct {
    #[yaserde(prefix="ns")]
    Item
  }

  impl Default for XmlStruct {
    fn default() -> XmlStruct {
      XmlStruct::Item
    }
  }

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><ns:root xmlns:ns=\"http://www.sample.com/ns/domain\">ns:Item</ns:root>";
  convert_and_validate!(content, XmlStruct, XmlStruct::Item);
}
