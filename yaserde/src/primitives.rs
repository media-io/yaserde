use std::{io::Read, io::Write};

use crate::{de, ser};

pub fn serialize_primitives<S, W: Write>(
  self_bypass: &S,
  default_name: &str,
  writer: &mut ser::Serializer<W>,
  serialize_function: impl FnOnce(&S) -> String,
) -> Result<(), String> {
  let name = writer
    .get_start_event_name()
    .unwrap_or_else(|| default_name.to_string());

  if !writer.skip_start_end() {
    writer
      .write(xml::writer::XmlEvent::start_element(name.as_str()))
      .map_err(|_e| format!("Start element {name:?} write failed"))?;
  }

  writer
    .write(xml::writer::XmlEvent::characters(
      serialize_function(self_bypass).as_str(),
    ))
    .map_err(|_e| format!("Element value {name:?} write failed"))?;

  if !writer.skip_start_end() {
    writer
      .write(xml::writer::XmlEvent::end_element())
      .map_err(|_e| format!("End element {name:?} write failed"))?;
  }

  Ok(())
}

pub fn deserialize_primitives<S, R: Read>(
  reader: &mut de::Deserializer<R>,
  deserialize_function: impl FnOnce(&str) -> Result<S, String>,
) -> Result<S, String> {
  if let Ok(xml::reader::XmlEvent::StartElement { .. }) = reader.peek() {
    reader.next_event()?;
  } else {
    return Err("Start element not found".to_string());
  }

  if let Ok(xml::reader::XmlEvent::Characters(ref text)) = reader.peek() {
    deserialize_function(text)
  } else {
    deserialize_function("")
  }
}
