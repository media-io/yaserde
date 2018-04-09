
extern crate xml;
#[cfg(feature = "yaserde_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate yaserde_derive;

use std::io::Read;
use xml::EventReader;
use xml::attribute::OwnedAttribute;

pub trait YaDeserialize : Sized {
  fn derive_deserialize<R: Read>(read: &mut EventReader<R>, parent_attributes: Option<&Vec<OwnedAttribute>>) -> Result<Self, String>;
}

pub trait YaSerialize {
  fn derive_serialize();
}
