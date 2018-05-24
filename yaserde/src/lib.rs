#[macro_use]
extern crate log;
extern crate xml;
#[cfg(feature = "yaserde_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate yaserde_derive;

use std::io::{Read, Write};
use xml::writer::XmlEvent;

pub mod de;
pub mod ser;

pub trait YaDeserialize: Sized {
  fn deserialize<R: Read>(reader: &mut de::Deserializer<R>) -> Result<Self, String>;
}

pub trait YaSerialize: Sized {
  fn serialize<W: Write>(&self, writer: &mut ser::Serializer<W>) -> Result<(), String>;
}

pub trait Visitor<'de>: Sized {
  /// The value produced by this visitor.
  type Value;

  fn visit_bool(self, v: &str) -> Result<Self::Value, String> {
    Err(format!("Unexpected bool {:?}", v))
  }

  fn visit_i8(self, v: &str) -> Result<Self::Value, String> {
    Err(format!("Unexpected i8 {:?}", v))
  }

  fn visit_u8(self, v: &str) -> Result<Self::Value, String> {
    Err(format!("Unexpected u8 {:?}", v))
  }

  fn visit_i16(self, v: &str) -> Result<Self::Value, String> {
    Err(format!("Unexpected i16 {:?}", v))
  }

  fn visit_u16(self, v: &str) -> Result<Self::Value, String> {
    Err(format!("Unexpected u16 {:?}", v))
  }

  fn visit_i32(self, v: &str) -> Result<Self::Value, String> {
    Err(format!("Unexpected i32 {:?}", v))
  }

  fn visit_u32(self, v: &str) -> Result<Self::Value, String> {
    Err(format!("Unexpected u32 {:?}", v))
  }

  fn visit_i64(self, v: &str) -> Result<Self::Value, String> {
    Err(format!("Unexpected i64 {:?}", v))
  }

  fn visit_u64(self, v: &str) -> Result<Self::Value, String> {
    Err(format!("Unexpected u64 {:?}", v))
  }

  fn visit_str(self, v: &str) -> Result<Self::Value, String> {
    Err(format!("Unexpected str {:?}", v))
  }

  #[inline]
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn visit_string<String>(self, v: String) -> Result<Self::Value, String> {
    self.visit_str(&v)
  }
}

macro_rules! serialize_type {
  ($type: ty) => {
    impl YaSerialize for $type {
      fn serialize<W: Write>(&self, writer: &mut ser::Serializer<W>) -> Result<(), String> {
        let content = format!("{}", self);
        let event = XmlEvent::characters(&content);
        let _ret = writer.write(event);
        Ok(())
      }
    }
  };
}

serialize_type!(bool);
serialize_type!(char);

serialize_type!(usize);
serialize_type!(u8);
serialize_type!(u16);
serialize_type!(u32);
serialize_type!(u64);

serialize_type!(isize);
serialize_type!(i8);
serialize_type!(i16);
serialize_type!(i32);
serialize_type!(i64);

serialize_type!(f32);
serialize_type!(f64);

#[test]
fn default_visitor() {
  struct Test;
  impl<'de> Visitor<'de> for Test {
    type Value = u8;
  }

  let t = Test{};
  assert_eq!(t.visit_bool(""), Err("Unexpected bool \"\"".to_string()));
  assert_eq!(t.visit_i8(""), Err("Unexpected i8 \"\"".to_string()));
  assert_eq!(t.visit_u8(""), Err("Unexpected u8 \"\"".to_string()));
  assert_eq!(t.visit_i16(""), Err("Unexpected i16 \"\"".to_string()));
  assert_eq!(t.visit_u16(""), Err("Unexpected u16 \"\"".to_string()));
  assert_eq!(t.visit_i32(""), Err("Unexpected i32 \"\"".to_string()));
  assert_eq!(t.visit_u32(""), Err("Unexpected u32 \"\"".to_string()));
  assert_eq!(t.visit_i64(""), Err("Unexpected i64 \"\"".to_string()));
  assert_eq!(t.visit_u64(""), Err("Unexpected u64 \"\"".to_string()));
  assert_eq!(t.visit_str(""), Err("Unexpected str \"\"".to_string()));
}
