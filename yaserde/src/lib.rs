
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

pub trait YaDeserialize : Sized {
  fn deserialize<R: Read>(reader: &mut de::Deserializer<R>) -> Result<Self, String>;
}

pub trait YaSerialize : Sized {
  fn serialize<W: Write>(&self, writer: &mut ser::Serializer<W>) -> Result<(), String>;
}

pub trait Visitor<'de>: Sized {
  /// The value produced by this visitor.
  type Value;

  fn visit_str(self, v: &str) -> Result<Self::Value, String>
  {
    Err(format!("Unexpected str {}", v))
  }

  #[inline]
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn visit_string<String>(self, v: String) -> Result<Self::Value, String>
  {
    self.visit_str(&v)
  }
}

macro_rules! serialize_type {
  ($type:ty) => (
    impl YaSerialize for $type {
      fn serialize<W: Write>(&self, writer: &mut ser::Serializer<W>) -> Result<(), String> {
        let content = format!("{}", self);
        let event = XmlEvent::characters(&content);
        let _ret = writer.write(event);
        Ok(())
      }
    }
  )
}

serialize_type!(bool);
serialize_type!(char);

serialize_type!(u8);
serialize_type!(u16);
serialize_type!(u32);
serialize_type!(u64);

serialize_type!(i8);
serialize_type!(i16);
serialize_type!(i32);
serialize_type!(i64);

serialize_type!(f32);
serialize_type!(f64);

