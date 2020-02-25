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
  ($type:ty, $value:expr, $content:expr) => {{
    #[derive(YaDeserialize, PartialEq, Debug)]
    #[yaserde(root = "data")]
    pub struct Data {
      item: $type,
    }

    let model = Data { item: $value };

    let content = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?><data><item>")
      + $content
      + "</item></data>";

    let loaded: Result<Data, String> = from_str(&content);
    assert_eq!(loaded, Ok(model));
  }};
}

macro_rules! convert_and_validate_for_attribute {
  ($type:ty, $value:expr, $content:expr) => {{
    #[derive(YaDeserialize, PartialEq, Debug)]
    #[yaserde(root = "data")]
    pub struct Data {
      #[yaserde(attribute)]
      item: $type,
    }

    let model = Data { item: $value };

    let content =
      String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?><data item=\"") + $content + "\" />";

    let loaded: Result<Data, String> = from_str(&content);
    assert_eq!(loaded, Ok(model));
  }};
}

#[test]
fn de_type() {
  convert_and_validate!(String, "test".to_string(), "test");
  convert_and_validate!(bool, true, "true");
  convert_and_validate!(u8, 12 as u8, "12");
  convert_and_validate!(i8, 12 as i8, "12");
  convert_and_validate!(i8, -12 as i8, "-12");
  convert_and_validate!(u16, 12 as u16, "12");
  convert_and_validate!(i16, 12 as i16, "12");
  convert_and_validate!(i16, -12 as i16, "-12");
  convert_and_validate!(u32, 12 as u32, "12");
  convert_and_validate!(i32, 12 as i32, "12");
  convert_and_validate!(i32, -12 as i32, "-12");
  convert_and_validate!(u64, 12 as u64, "12");
  convert_and_validate!(i64, 12 as i64, "12");
  convert_and_validate!(i64, -12 as i64, "-12");
  convert_and_validate!(f32, -12.5_f32 as f32, "-12.5");
  convert_and_validate!(f64, -12.5 as f64, "-12.5");

  convert_and_validate_for_attribute!(String, "test".to_string(), "test");
  convert_and_validate_for_attribute!(bool, true, "true");
  convert_and_validate_for_attribute!(u8, 12 as u8, "12");
  convert_and_validate_for_attribute!(i8, 12 as i8, "12");
  convert_and_validate_for_attribute!(i8, -12 as i8, "-12");
  convert_and_validate_for_attribute!(u16, 12 as u16, "12");
  convert_and_validate_for_attribute!(i16, 12 as i16, "12");
  convert_and_validate_for_attribute!(i16, -12 as i16, "-12");
  convert_and_validate_for_attribute!(u32, 12 as u32, "12");
  convert_and_validate_for_attribute!(i32, 12 as i32, "12");
  convert_and_validate_for_attribute!(i32, -12 as i32, "-12");
  convert_and_validate_for_attribute!(u64, 12 as u64, "12");
  convert_and_validate_for_attribute!(i64, 12 as i64, "12");
  convert_and_validate_for_attribute!(i64, -12 as i64, "-12");
  convert_and_validate_for_attribute!(f32, -12.5 as f32, "-12.5");
  convert_and_validate_for_attribute!(f64, -12.5 as f64, "-12.5");
}
