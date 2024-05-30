use std::{io::Read, io::Write};

use crate::{de, ser};

pub fn serialize_primitives<S, W: Write>(
    self_bypass: &S,
    default_name: &str,
    writer: &mut ser::Serializer<W>,
    ser_fn: impl FnOnce(&S) -> String,
) -> Result<(), String> {
    let name = writer
        .get_start_event_name()
        .unwrap_or_else(|| default_name.to_string());

    if !writer.skip_start_end() {
        writer
            .write(xml::writer::XmlEvent::start_element(name.as_str()))
            .map_err(|_e| "Start element write failed".to_string())?;
    }

    writer
        .write(xml::writer::XmlEvent::characters(
            ser_fn(self_bypass).as_str(),
        ))
        .map_err(|_e| "Element value write failed".to_string())?;

    if !writer.skip_start_end() {
        writer
            .write(xml::writer::XmlEvent::end_element())
            .map_err(|_e| "End element write failed".to_string())?;
    }

    Ok(())
}

pub fn deserialize_primitives<S, R: Read>(
    reader: &mut de::Deserializer<R>,
    de_fn: impl FnOnce(&str) -> Result<S, String>,
) -> Result<S, String> {
    if let Ok(xml::reader::XmlEvent::StartElement { .. }) = reader.peek() {
        reader.next_event()?;
    } else {
        return Err("Start element not found".to_string());
    }

    if let Ok(xml::reader::XmlEvent::Characters(ref text)) = reader.peek() {
        de_fn(text)
    } else {
        de_fn("")
    }
}
