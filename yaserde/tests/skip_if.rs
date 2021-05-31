#[macro_use]
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

fn init() {
  let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn skip_serializing_if_for_struct() {
  init();

  fn default_string_function() -> String {
    "mask_default".to_string()
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    #[yaserde(
      skip_serializing_if = "check_string_function",
      default = "default_string_function"
    )]
    string_with_default_item: String,
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
    fn check_string_function(&self, value: &str) -> bool {
      value == "something"
    }

    fn check_option_string_function(&self, value: &Option<String>) -> bool {
      value == &Some("something".to_string())
    }

    fn check_bool_function(&self, value: &bool) -> bool {
      value == &true
    }

    fn check_f32_function(&self, value: &f32) -> bool {
      (value - 0.0).abs() < std::f32::EPSILON
    }
  }

  let model = XmlStruct {
    string_with_default_item: "mask_default".to_string(),
    string_item: "something".to_string(),
    bool_item: true,
    f32_item: 0.0,
    option_string_item: Some("something".to_string()),
  };

  let content = "<base />";
  serialize_and_validate!(model, content);
}

#[test]
fn skip_serializing_if_for_struct_attributes() {
  init();

  fn default_string_function() -> String {
    "mask_default".to_string()
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(rename = "base")]
  pub struct XmlStruct {
    #[yaserde(
      attribute,
      skip_serializing_if = "check_string_function",
      default = "default_string_function"
    )]
    string_with_default_item: String,
    #[yaserde(attribute, skip_serializing_if = "check_string_function")]
    string_item: String,
    #[yaserde(attribute, skip_serializing_if = "check_bool_function")]
    bool_item: bool,
    #[yaserde(attribute, skip_serializing_if = "check_f32_function")]
    f32_item: f32,
    #[yaserde(attribute, skip_serializing_if = "check_option_string_function")]
    option_string_item: Option<String>,
  }

  impl XmlStruct {
    fn check_string_function(&self, value: &str) -> bool {
      value == "something"
    }

    fn check_option_string_function(&self, value: &Option<String>) -> bool {
      value == &Some("something".to_string())
    }

    fn check_bool_function(&self, value: &bool) -> bool {
      value == &true
    }

    fn check_f32_function(&self, value: &f32) -> bool {
      (value - 0.0).abs() < std::f32::EPSILON
    }
  }

  let model = XmlStruct {
    string_with_default_item: "mask_default".to_string(),
    string_item: "something".to_string(),
    bool_item: true,
    f32_item: 0.0,
    option_string_item: Some("something".to_string()),
  };

  let content = "<base />";
  serialize_and_validate!(model, content);
}
