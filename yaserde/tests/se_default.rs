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
fn se_default_field_string() {
  fn default_string() -> String {
    "my_default_value".to_string()
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(default = "default_string")]
    background: String,
  }

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base />";
  convert_and_validate!(
    XmlStruct {
      background: "my_default_value".to_string(),
    },
    content
  );
  let content =
    "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><background>my_value</background></base>";
  convert_and_validate!(
    XmlStruct {
      background: "my_value".to_string(),
    },
    content
  );
}

#[test]
fn se_default_field_boolean() {
  fn default_boolean() -> bool {
    true
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(default = "default_boolean")]
    background: bool,
  }

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base />";
  convert_and_validate!(XmlStruct { background: true }, content);

  let content =
    "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><background>false</background></base>";
  convert_and_validate!(XmlStruct { background: false }, content);
}

#[test]
fn se_default_field_number() {
  fn default_number() -> u8 {
    6
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(default = "default_number")]
    background: u8,
  }

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base />";
  convert_and_validate!(XmlStruct { background: 6 }, content);
}

#[test]
fn se_default_attribute_string() {
  fn default_string() -> String {
    "my_default_value".to_string()
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute, default = "default_string")]
    background: String,
  }

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base />";
  convert_and_validate!(
    XmlStruct {
      background: "my_default_value".to_string(),
    },
    content
  );
}
