#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::io::Read;
use yaserde::de::from_str;
use yaserde::YaDeserialize;

macro_rules! convert_and_validate {
  ($content: expr, $struct: tt, $model: expr) => {
    let loaded: Result<$struct, String> = from_str($content);
    assert_eq!(loaded, Ok($model));
  };
}

#[test]
fn de_default_field_string() {
  fn default_string() -> String {
    "my_default_value".to_string()
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(default = "default_string")]
    background: String,
  }

  let content =
    "<?xml version=\"1.0\" encoding=\"utf-8\"?><base></base>";
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: "my_default_value".to_string(),
    }
  );
}

#[test]
fn de_default_field_boolean() {
  fn default_boolean() -> bool {
    true
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(default = "default_boolean")]
    background: bool,
  }

  let content =
    "<?xml version=\"1.0\" encoding=\"utf-8\"?><base></base>";
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: true,
    }
  );
}

#[test]
fn de_default_field_number() {
  fn default_number() -> u8 {
    6
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(default = "default_number")]
    background: u8,
  }

  let content =
    "<?xml version=\"1.0\" encoding=\"utf-8\"?><base></base>";
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: 6,
    }
  );
}

#[test]
fn de_default_attribute_string() {
  fn default_string() -> String {
    "my_default_value".to_string()
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute, default = "default_string")]
    background: String,
  }

  let content =
    "<?xml version=\"1.0\" encoding=\"utf-8\"?><base></base>";
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: "my_default_value".to_string(),
    }
  );
}
