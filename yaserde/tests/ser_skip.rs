#[macro_use]
extern crate yaserde_derive;

use std::io::Write;
use yaserde::ser::to_string;
use yaserde::YaSerialize;

macro_rules! convert_and_validate {
  ($model: expr, $content: expr) => {
    let data: Result<String, String> = to_string(&$model);
    assert_eq!(
      data,
      Ok(
        String::from($content)
          .split("\n")
          .map(|s| s.trim())
          .collect::<String>()
      )
    );
  };
}

#[test]
fn ser_skip_serializing_if_for_struct() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(skip_serializing_if = "check_string_function")]
    string_item: String,
    #[yaserde(skip_serializing_if = "check_bool_function")]
    bool_item: bool,
    #[yaserde(skip_serializing_if = "check_f32_function")]
    f32_item: f32,
    #[yaserde(skip_serializing_if = "check_option_string_function")]
    option_string_item: Option<String>,
  }

  impl XmlStruct {
    fn check_string_function(&self, value: &String) -> bool {
      value == "something"
    }

    fn check_option_string_function(&self, value: &Option<String>) -> bool {
      value == &Some("something".to_string())
    }

    fn check_bool_function(&self, value: &bool) -> bool {
      value == &true
    }

    fn check_f32_function(&self, value: &f32) -> bool {
      value == &0.0
    }
  }

  let model = XmlStruct {
    string_item: "something".to_string(),
    bool_item: true,
    f32_item: 0.0,
    option_string_item: Some("something".to_string()),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base />";
  convert_and_validate!(model, content);
}
