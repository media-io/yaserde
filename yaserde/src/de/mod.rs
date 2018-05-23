use std::io::Read;
use xml::reader::{EventReader, ParserConfig, XmlEvent};
use xml::name::OwnedName;
use YaDeserialize;

pub fn from_str<T: YaDeserialize>(s: &str) -> Result<T, String> {
  from_reader(s.as_bytes())
}

pub fn from_reader<R: Read, T: YaDeserialize>(reader: R) -> Result<T, String> {
  T::deserialize(&mut Deserializer::new_from_reader(reader))
}

pub struct Deserializer<R: Read> {
  depth: usize,
  reader: EventReader<R>,
  peeked: Option<XmlEvent>,
  is_map_value: bool,
}

impl<'de, R: Read> Deserializer<R> {
  pub fn new(reader: EventReader<R>) -> Self {
    Deserializer {
      depth: 0,
      reader,
      peeked: None,
      is_map_value: false,
    }
  }

  pub fn new_from_reader(reader: R) -> Self {
    let config = ParserConfig::new()
      .trim_whitespace(true)
      .whitespace_to_characters(true)
      .cdata_to_characters(true)
      .ignore_comments(true)
      .coalesce_characters(true);

    Self::new(EventReader::new_with_config(reader, config))
  }

  pub fn peek(&mut self) -> Result<&XmlEvent, String> {
    if self.peeked.is_none() {
      self.peeked = Some(self.inner_next()?);
    }

    if let Some(ref next) = self.peeked {
      Ok(&next)
    } else {
      Err(String::from("unable to peek next item"))
    }
  }

  pub fn inner_next(&mut self) -> Result<XmlEvent, String> {
    loop {
      if let Ok(next) = self.reader.next() {
        match next {
          XmlEvent::StartDocument { .. }
          | XmlEvent::ProcessingInstruction { .. }
          | XmlEvent::Comment(_) => { /* skip */ }
          other => return Ok(other),
        }
      } else {
        println!("{:?}", self.peeked);
        return Err(String::from("bad content"));
      }
    }
  }

  pub fn next_event(&mut self) -> Result<XmlEvent, String> {
    let next_event = if let Some(peeked) = self.peeked.take() {
      peeked
    } else {
      self.inner_next()?
    };
    match next_event {
      XmlEvent::StartElement { .. } => {
        self.depth += 1;
      }
      XmlEvent::EndElement { .. } => {
        self.depth -= 1;
      }
      _ => {}
    }
    debug!("Fetched {:?}", next_event);
    Ok(next_event)
  }

  pub fn set_map_value(&mut self) {
    self.is_map_value = true;
  }

  pub fn unset_map_value(&mut self) -> bool {
    ::std::mem::replace(&mut self.is_map_value, false)
  }

  pub fn read_inner_value<T, F: FnOnce(&mut Self) -> Result<T, String>>(
    &mut self,
    f: F,
  ) -> Result<T, String> {
    if self.unset_map_value() {
      if let Ok(XmlEvent::StartElement { name, .. }) = self.next_event() {
        let result = f(self)?;
        self.expect_end_element(&name)?;
        Ok(result)
      } else {
        Err("Internal error: Bad Event".to_string())
      }
    } else {
      f(self)
    }
  }

  pub fn expect_end_element(&mut self, start_name: &OwnedName) -> Result<(), String> {
    if let XmlEvent::EndElement { name, .. } = self.next_event()? {
      if name == *start_name {
        Ok(())
      } else {
        Err(format!(
          "End tag </{}> didn't match the start tag <{}>",
          name.local_name, start_name.local_name
        ))
      }
    } else {
      Err(format!("Unexpected token </{}>", start_name.local_name))
    }
  }
}
