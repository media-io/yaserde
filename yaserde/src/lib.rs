
extern crate xml;
#[cfg(feature = "yaserde_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate yaserde_derive;

use std::io::{Read, Write};
use xml::{EventReader, EventWriter};
use xml::attribute::OwnedAttribute;

pub trait YaDeserialize : Sized {
  fn derive_deserialize<R: Read>(read: &mut EventReader<R>, parent_attributes: Option<&Vec<OwnedAttribute>>) -> Result<Self, String>;
}

pub trait YaSerialize : Sized {
  fn derive_serialize<W: Write>(&self, read: &mut EventWriter<W>, skip_start_end: bool) -> Result<(), String>;
}
