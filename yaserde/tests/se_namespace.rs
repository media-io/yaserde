extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::io::Write;
use yaserde::ser::to_string;
use yaserde::YaSerialize;

macro_rules! convert_and_validate {
  ($model:expr, $content:expr) => {
    let data: Result<String, String> = to_string(&$model);
    assert_eq!(data, Ok(String::from($content)));
  };
}

#[test]
fn ser_struct_namespace() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(
    root = "root",
    prefix = "ns",
    namespace = "ns: http://www.sample.com/ns/domain"
  )]
  pub struct XmlStruct {
    #[yaserde(prefix = "ns")]
    item: String,
  }

  let model = XmlStruct {
    item: "something".to_string(),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><ns:root xmlns:ns=\"http://www.sample.com/ns/domain\"><ns:item>something</ns:item></ns:root>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_enum_namespace() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(
    root = "root",
    prefix = "ns",
    namespace = "ns: http://www.sample.com/ns/domain"
  )]
  pub enum XmlStruct {
    #[yaserde(prefix = "ns")]
    Item,
  }

  let model = XmlStruct::Item;

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><ns:root xmlns:ns=\"http://www.sample.com/ns/domain\">ns:Item</ns:root>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_struct_multi_namespace() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(
    root = "root",
    namespace = "ns1: http://www.sample.com/ns/domain1",
    namespace = "ns2: http://www.sample.com/ns/domain2"
  )]
  pub struct XmlStruct {
    #[yaserde(prefix = "ns1")]
    item_1: String,
    #[yaserde(prefix = "ns2")]
    item_2: String,
  }

  let model = XmlStruct {
    item_1: "something 1".to_string(),
    item_2: "something 2".to_string(),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><root xmlns:ns1=\"http://www.sample.com/ns/domain1\" xmlns:ns2=\"http://www.sample.com/ns/domain2\"><ns1:item_1>something 1</ns1:item_1><ns2:item_2>something 2</ns2:item_2></root>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_enum_multi_namespace() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(
    root = "root",
    namespace = "ns1: http://www.sample.com/ns/domain1",
    namespace = "ns2: http://www.sample.com/ns/domain2"
  )]
  pub enum XmlStruct {
    #[yaserde(prefix = "ns1")]
    Item1,
    #[yaserde(prefix = "ns2")]
    Item2,
  }

  let model1 = XmlStruct::Item1;
  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><root xmlns:ns1=\"http://www.sample.com/ns/domain1\" xmlns:ns2=\"http://www.sample.com/ns/domain2\">ns1:Item1</root>";
  convert_and_validate!(model1, content);
  let model2 = XmlStruct::Item2;
  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><root xmlns:ns1=\"http://www.sample.com/ns/domain1\" xmlns:ns2=\"http://www.sample.com/ns/domain2\">ns2:Item2</root>";
  convert_and_validate!(model2, content);
}

#[test]
fn ser_struct_attribute_namespace() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(
    root = "root",
    namespace = "ns1: http://www.sample.com/ns/domain1",
    namespace = "ns2: http://www.sample.com/ns/domain2"
  )]
  pub struct XmlStruct {
    #[yaserde(prefix = "ns1")]
    item_1: String,
    #[yaserde(attribute, prefix = "ns2")]
    item_2: String,
  }

  let model = XmlStruct {
    item_1: "something 1".to_string(),
    item_2: "something 2".to_string(),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><root xmlns:ns1=\"http://www.sample.com/ns/domain1\" xmlns:ns2=\"http://www.sample.com/ns/domain2\" ns2:item_2=\"something 2\"><ns1:item_1>something 1</ns1:item_1></root>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_struct_default_namespace() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(
    root = "tt",
    namespace = "http://www.w3.org/ns/ttml",
    namespace = "ttm: http://www.w3.org/ns/ttml#metadata"
  )]
  pub struct XmlStruct {
    item: String,
  }

  let model = XmlStruct {
    item: "something".to_string(),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><tt xmlns=\"http://www.w3.org/ns/ttml\" xmlns:ttm=\"http://www.w3.org/ns/ttml#metadata\"><item>something</item></tt>";
  convert_and_validate!(model, content);
}
