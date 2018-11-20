use std::io::{Cursor, Write};
use std::str;
use xml;
use xml::writer::XmlEvent;
use xml::{EmitterConfig, EventWriter};
use YaSerialize;

pub fn to_string<T: YaSerialize>(model: &T) -> Result<String, String> {
  let buf = Cursor::new(Vec::new());
  let cursor = serialize_with_writer(model, buf)?;
  let data = str::from_utf8(cursor.get_ref()).expect("Found invalid UTF-8");
  Ok(String::from(data))
}

pub fn serialize_with_writer<W: Write, T: YaSerialize>(model: &T, writer: W) -> Result<W, String> {
  let mut serializer = Serializer::new_from_writer(writer);
  match model.serialize(&mut serializer) {
    Ok(()) => Ok(serializer.into_inner()),
    Err(msg) => Err(msg),
  }
}

pub fn to_string_content<T: YaSerialize>(model: &T) -> Result<String, String> {
  let buf = Cursor::new(Vec::new());
  let cursor = serialize_with_writer_content(model, buf)?;
  let data = str::from_utf8(cursor.get_ref()).expect("Found invalid UTF-8");
  Ok(String::from(data))
}

pub fn serialize_with_writer_content<W: Write, T: YaSerialize>(
  model: &T,
  writer: W,
) -> Result<W, String> {
  let mut serializer = Serializer::new_for_inner(writer);
  serializer.set_skip_start_end(true);
  match model.serialize(&mut serializer) {
    Ok(()) => Ok(serializer.into_inner()),
    Err(msg) => Err(msg),
  }
}

pub struct Serializer<W: Write> {
  writer: EventWriter<W>,
  skip_start_end: bool,
  start_event_name: Option<String>,
}

impl<'de, W: Write> Serializer<W> {
  pub fn new(writer: EventWriter<W>) -> Self {
    Serializer {
      writer,
      skip_start_end: false,
      start_event_name: None,
    }
  }

  pub fn new_from_writer(writer: W) -> Self {
    let config = EmitterConfig::new().cdata_to_characters(true);

    Self::new(EventWriter::new_with_config(writer, config))
  }

  pub fn new_for_inner(writer: W) -> Self {
    let config = EmitterConfig::new().write_document_declaration(false);

    Self::new(EventWriter::new_with_config(writer, config))
  }

  pub fn into_inner(self) -> W {
    self.writer.into_inner()
  }

  pub fn skip_start_end(&self) -> bool {
    self.skip_start_end
  }

  pub fn set_skip_start_end(&mut self, state: bool) {
    self.skip_start_end = state;
  }

  pub fn get_start_event_name<'a>(&self) -> Option<String> {
    self.start_event_name.clone()
  }

  pub fn set_start_event_name<'a>(&mut self, name: Option<String>) {
    self.start_event_name = name;
  }

  pub fn write<'a, E>(&mut self, event: E) -> xml::writer::Result<()>
  where
    E: Into<XmlEvent<'a>>,
  {
    self.writer.write(event)
  }
}
