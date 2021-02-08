#[macro_use]
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

fn init() {
  let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn ser_type() {
  init();

  test_for_type!(String, "test".to_string(), Some("test"));
  test_for_type!(bool, true, Some("true"));
  test_for_type!(u8, 12_u8, Some("12"));
  test_for_type!(i8, 12_i8, Some("12"));
  test_for_type!(i8, -12_i8, Some("-12"));
  test_for_type!(u16, 12_u16, Some("12"));
  test_for_type!(i16, 12_i16, Some("12"));
  test_for_type!(i16, -12_i16, Some("-12"));
  test_for_type!(u32, 12_u32, Some("12"));
  test_for_type!(i32, 12_i32, Some("12"));
  test_for_type!(i32, -12_i32, Some("-12"));
  test_for_type!(u64, 12_u64, Some("12"));
  test_for_type!(i64, 12_i64, Some("12"));
  test_for_type!(i64, -12_i64, Some("-12"));
  test_for_type!(f32, -12.5_f32, Some("-12.5"));
  test_for_type!(f64, -12.5_f64, Some("-12.5"));
  test_for_type!(Vec::<String>, vec![], None);
  test_for_type!(Vec::<String>, vec!["test".to_string()], Some("test"));

  test_for_attribute_type!(String, "test".to_string(), Some("test"));
  test_for_attribute_type!(bool, true, Some("true"));
  test_for_attribute_type!(u8, 12_u8, Some("12"));
  test_for_attribute_type!(i8, 12_i8, Some("12"));
  test_for_attribute_type!(i8, -12_i8, Some("-12"));
  test_for_attribute_type!(u16, 12_u16, Some("12"));
  test_for_attribute_type!(i16, 12_i16, Some("12"));
  test_for_attribute_type!(i16, -12_i16, Some("-12"));
  test_for_attribute_type!(u32, 12_u32, Some("12"));
  test_for_attribute_type!(i32, 12_i32, Some("12"));
  test_for_attribute_type!(i32, -12_i32, Some("-12"));
  test_for_attribute_type!(u64, 12_u64, Some("12"));
  test_for_attribute_type!(i64, 12_i64, Some("12"));
  test_for_attribute_type!(i64, -12_i64, Some("-12"));
  test_for_attribute_type!(f32, -12.5_f32, Some("-12.5"));
  test_for_attribute_type!(f64, -12.5_f64, Some("-12.5"));
}
