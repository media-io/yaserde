#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::io::Write;
use yaserde::YaSerialize;
use yaserde::ser::to_string;

macro_rules! convert_and_validate {
  ($type: ty, $value: expr, $content: expr) => {{
    #[derive(YaSerialize, PartialEq, Debug)]
    #[yaserde(root = "data")]
    pub struct Data {
      item: Option<$type>,
    }
    let model = Data { item: $value };

    let data: Result<String, String> = to_string(&model);

    let content = if let Some(str_value) = $content {
      "<data><item>".to_string() + str_value + "</item></data>"
    } else {
      "<data />".to_string()
    };

    let content = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?>") + &content;
    assert_eq!(data, Ok(content));
  }};
}

macro_rules! convert_and_validate_as_attribute {
  ($type: ty, $value: expr, $content: expr) => {{
    #[derive(YaSerialize, PartialEq, Debug)]
    #[yaserde(root = "data")]
    pub struct Data {
      #[yaserde(attribute)]
      item: Option<$type>,
    }
    let model = Data { item: $value };

    let data: Result<String, String> = to_string(&model);

    let content = if let Some(str_value) = $content {
      "<data item=\"".to_string() + str_value + "\" />"
    } else {
      "<data />".to_string()
    };

    let content = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?>") + &content;
    assert_eq!(data, Ok(content));
  }};
}

#[test]
fn ser_option() {
  convert_and_validate!(String, Some("test".to_string()), Some("test"));
  convert_and_validate!(String, None, None);
  convert_and_validate!(bool, Some(true), Some("true"));
  convert_and_validate!(bool, None, None);
  convert_and_validate!(u8, Some(12 as u8), Some("12"));
  convert_and_validate!(u8, None, None);
  convert_and_validate!(i8, Some(12 as i8), Some("12"));
  convert_and_validate!(i8, Some(-12 as i8), Some("-12"));
  convert_and_validate!(i8, None, None);
  convert_and_validate!(u16, Some(12 as u16), Some("12"));
  convert_and_validate!(u16, None, None);
  convert_and_validate!(i16, Some(12 as i16), Some("12"));
  convert_and_validate!(i16, Some(-12 as i16), Some("-12"));
  convert_and_validate!(i16, None, None);
  convert_and_validate!(u32, Some(12 as u32), Some("12"));
  convert_and_validate!(u32, None, None);
  convert_and_validate!(i32, Some(12 as i32), Some("12"));
  convert_and_validate!(i32, Some(-12 as i32), Some("-12"));
  convert_and_validate!(i32, None, None);
  convert_and_validate!(u64, Some(12 as u64), Some("12"));
  convert_and_validate!(u64, None, None);
  convert_and_validate!(i64, Some(12 as i64), Some("12"));
  convert_and_validate!(i64, Some(-12 as i64), Some("-12"));
  convert_and_validate!(i64, None, None);
  convert_and_validate!(f32, Some(-12.5 as f32), Some("-12.5"));
  convert_and_validate!(f32, None, None);
  convert_and_validate!(f64, Some(-12.5 as f64), Some("-12.5"));
  convert_and_validate!(f64, None, None);

  convert_and_validate_as_attribute!(String, Some("test".to_string()), Some("test"));
  convert_and_validate_as_attribute!(String, None, None);
  convert_and_validate_as_attribute!(bool, Some(true), Some("true"));
  convert_and_validate_as_attribute!(bool, None, None);
  convert_and_validate_as_attribute!(u8, Some(12 as u8), Some("12"));
  convert_and_validate_as_attribute!(u8, None, None);
  convert_and_validate_as_attribute!(i8, Some(12 as i8), Some("12"));
  convert_and_validate_as_attribute!(i8, Some(-12 as i8), Some("-12"));
  convert_and_validate_as_attribute!(i8, None, None);
  convert_and_validate_as_attribute!(u16, Some(12 as u16), Some("12"));
  convert_and_validate_as_attribute!(u16, None, None);
  convert_and_validate_as_attribute!(i16, Some(12 as i16), Some("12"));
  convert_and_validate_as_attribute!(i16, Some(-12 as i16), Some("-12"));
  convert_and_validate_as_attribute!(i16, None, None);
  convert_and_validate_as_attribute!(u32, Some(12 as u32), Some("12"));
  convert_and_validate_as_attribute!(u32, None, None);
  convert_and_validate_as_attribute!(i32, Some(12 as i32), Some("12"));
  convert_and_validate_as_attribute!(i32, Some(-12 as i32), Some("-12"));
  convert_and_validate_as_attribute!(i32, None, None);
  convert_and_validate_as_attribute!(u64, Some(12 as u64), Some("12"));
  convert_and_validate_as_attribute!(u64, None, None);
  convert_and_validate_as_attribute!(i64, Some(12 as i64), Some("12"));
  convert_and_validate_as_attribute!(i64, Some(-12 as i64), Some("-12"));
  convert_and_validate_as_attribute!(i64, None, None);
  convert_and_validate_as_attribute!(f32, Some(-12.5 as f32), Some("-12.5"));
  convert_and_validate_as_attribute!(f32, None, None);
  convert_and_validate_as_attribute!(f64, Some(-12.5 as f64), Some("-12.5"));
  convert_and_validate_as_attribute!(f64, None, None);
}
