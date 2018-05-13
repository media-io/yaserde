
#[macro_use]
extern crate log;
extern crate xml;
#[cfg(feature = "yaserde_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate yaserde_derive;

use std::io::{Read, Write};

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
