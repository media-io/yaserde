#[macro_use]
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

#[test]
fn default_field_string() {
  fn default_string() -> String {
    "my_default_value".to_string()
  }

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    #[yaserde(default = "default_string")]
    background: String,
  }

  let model = XmlStruct {
    background: "my_default_value".to_string(),
  };

  let content = "<base />";
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let content = "<base><background>my_value</background></base>";
  let model = XmlStruct {
    background: "my_value".to_string(),
  };
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);
}

#[test]
fn default_field_boolean() {
  fn default_boolean() -> bool {
    true
  }

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    #[yaserde(default = "default_boolean")]
    background: bool,
  }

  let content = "<base />";
  let model = XmlStruct { background: true };
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let content = "<base><background>false</background></base>";
  let model = XmlStruct { background: false };
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);
}

#[test]
fn default_field_number() {
  fn default_number() -> u8 {
    6
  }

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    #[yaserde(default = "default_number")]
    background: u8,
  }

  let content = "<base />";
  let model = XmlStruct { background: 6 };
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let content = "<base><background>4</background></base>";
  let model = XmlStruct { background: 4 };
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);
}

#[test]
fn default_attribute_string() {
  fn default_string() -> String {
    "my_default_value".to_string()
  }

  #[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute, default = "default_string")]
    background: String,
  }

  let content = "<base />";
  let model = XmlStruct {
    background: "my_default_value".to_string(),
  };
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);

  let content = r#"<base background="black" />"#;
  let model = XmlStruct {
    background: "black".to_string(),
  };
  serialize_and_validate!(model, content);
  deserialize_and_validate!(content, model, XmlStruct);
}
