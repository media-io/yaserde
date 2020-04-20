#[macro_use]
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

#[test]
fn ser_type() {
  test_for_type!(String, "test".to_string(), Some("test"));
  test_for_type!(bool, true, Some("true"));
  test_for_type!(u8, 12 as u8, Some("12"));
  test_for_type!(i8, 12 as i8, Some("12"));
  test_for_type!(i8, -12 as i8, Some("-12"));
  test_for_type!(u16, 12 as u16, Some("12"));
  test_for_type!(i16, 12 as i16, Some("12"));
  test_for_type!(i16, -12 as i16, Some("-12"));
  test_for_type!(u32, 12 as u32, Some("12"));
  test_for_type!(i32, 12 as i32, Some("12"));
  test_for_type!(i32, -12 as i32, Some("-12"));
  test_for_type!(u64, 12 as u64, Some("12"));
  test_for_type!(i64, 12 as i64, Some("12"));
  test_for_type!(i64, -12 as i64, Some("-12"));
  test_for_type!(f32, -12.5 as f32, Some("-12.5"));
  test_for_type!(f64, -12.5 as f64, Some("-12.5"));
  test_for_type!(Vec::<String>, vec![], None);
  test_for_type!(Vec::<String>, vec!["test".to_string()], Some("test"));

  test_for_attribute_type!(String, "test".to_string(), Some("test"));
  test_for_attribute_type!(bool, true, Some("true"));
  test_for_attribute_type!(u8, 12 as u8, Some("12"));
  test_for_attribute_type!(i8, 12 as i8, Some("12"));
  test_for_attribute_type!(i8, -12 as i8, Some("-12"));
  test_for_attribute_type!(u16, 12 as u16, Some("12"));
  test_for_attribute_type!(i16, 12 as i16, Some("12"));
  test_for_attribute_type!(i16, -12 as i16, Some("-12"));
  test_for_attribute_type!(u32, 12 as u32, Some("12"));
  test_for_attribute_type!(i32, 12 as i32, Some("12"));
  test_for_attribute_type!(i32, -12 as i32, Some("-12"));
  test_for_attribute_type!(u64, 12 as u64, Some("12"));
  test_for_attribute_type!(i64, 12 as i64, Some("12"));
  test_for_attribute_type!(i64, -12 as i64, Some("-12"));
  test_for_attribute_type!(f32, -12.5 as f32, Some("-12.5"));
  test_for_attribute_type!(f64, -12.5 as f64, Some("-12.5"));
}
