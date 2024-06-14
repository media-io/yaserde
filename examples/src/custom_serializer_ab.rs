use yaserde::{YaDeserialize, YaSerialize};

#[derive(Debug, PartialEq, YaDeserialize)]
struct RootElem {
  children: Vec<AB>,
}
impl YaSerialize for RootElem {
  fn serialize<W: std::io::prelude::Write>(
    &self,
    writer: &mut yaserde::ser::Serializer<W>,
  ) -> Result<(), String> {
    writer.write("Root {\n").unwrap();
    writer.write("/* A|B elements */\n").unwrap();
    for e in &self.children {
      e.serialize(writer).unwrap();
    }
    writer.write("}\n").unwrap();
    Ok(())
  }

  fn serialize_attributes(
    &self,
    attributes: Vec<xml::attribute::OwnedAttribute>,
    namespace: xml::namespace::Namespace,
  ) -> Result<
    (
      Vec<xml::attribute::OwnedAttribute>,
      xml::namespace::Namespace,
    ),
    String,
  > {
    Ok((attributes, namespace))
  }
}

#[derive(Debug, PartialEq)]
// #[derive(YaDeserialize)]
// #[yaserde(rename = "a")]
struct A {
  // #[yaserde(attribute)]
  attr: String,
}
impl YaSerialize for A {
  fn serialize<W: std::io::prelude::Write>(
    &self,
    writer: &mut yaserde::ser::Serializer<W>,
  ) -> Result<(), String> {
    writer.write("A {").unwrap();
    writer
      .write(format!("attr: {:?},", self.attr.as_str()).as_str())
      .unwrap();
    writer.write("}\n").unwrap();
    Ok(())
  }

  fn serialize_attributes(
    &self,
    attributes: Vec<xml::attribute::OwnedAttribute>,
    namespace: xml::namespace::Namespace,
  ) -> Result<
    (
      Vec<xml::attribute::OwnedAttribute>,
      xml::namespace::Namespace,
    ),
    String,
  > {
    Ok((attributes, namespace))
  }
}

#[derive(Debug, PartialEq)]
// #[derive(YaDeserialize)]
// #[yaserde(rename = "b")]
struct B {}
impl YaSerialize for B {
  fn serialize<W: std::io::prelude::Write>(
    &self,
    writer: &mut yaserde::ser::Serializer<W>,
  ) -> Result<(), String> {
    writer.write("B {}\n").unwrap();
    Ok(())
  }

  fn serialize_attributes(
    &self,
    attributes: Vec<xml::attribute::OwnedAttribute>,
    namespace: xml::namespace::Namespace,
  ) -> Result<
    (
      Vec<xml::attribute::OwnedAttribute>,
      xml::namespace::Namespace,
    ),
    String,
  > {
    Ok((attributes, namespace))
  }
}

#[derive(Debug, PartialEq)]
// #[derive(Default)]
enum AB {
  // #[default]
  // None,
  A(A),
  B(B),
}
impl Default for AB {
  fn default() -> Self {
    // NOTE: for debugging only; will be None
    Self::A(A {
      attr: String::from("undefined"),
    })
  }
}
impl YaDeserialize for AB {
  fn deserialize<R: std::io::Read>(
    reader: &mut yaserde::de::Deserializer<R>,
  ) -> Result<Self, String> {
    // dispatch to YaDeserialize for element
    print!(" - - - - -DISPATCHING");
    if let xml::reader::XmlEvent::StartElement { name, .. } = reader.peek()?.to_owned() {
      match name.local_name.as_str() {
        "a" => {
          let deserialized = A { attr: String::from("NOT DESERIALIZED")}; // TODO: A::deserialize(reader)?;
          return Ok(Self::A(deserialized));
        }
        "b" => {
          let deserialized = B {}; // TODO: B::deserialize(reader)?;
          return Ok(Self::B(deserialized));
        }
        _ => (),
      }
    }
    Err(format!("Expected a StartElement"))
  }
}
impl YaSerialize for AB {
  fn serialize<W: std::io::prelude::Write>(
    &self,
    writer: &mut yaserde::ser::Serializer<W>,
  ) -> Result<(), String> {
    writer.write("/* serialized AB */\n").unwrap();
    match self {
      // Self::None => {
      //   writer.write("UndefinedAB").unwrap();
      //   // return Err(format!("None element cannot be serialized"));
      // }
      Self::A(a) => {
        a.serialize(writer).unwrap();
      }
      Self::B(b) => {
        b.serialize(writer).unwrap();
      }
    }
    writer.write("}\n").unwrap();
    Ok(())
  }

  fn serialize_attributes(
    &self,
    attributes: Vec<xml::attribute::OwnedAttribute>,
    namespace: xml::namespace::Namespace,
  ) -> Result<
    (
      Vec<xml::attribute::OwnedAttribute>,
      xml::namespace::Namespace,
    ),
    String,
  > {
    Ok((attributes, namespace))
  }
}

#[test]
fn serialize_ab() {
  use std::fs;

  let content =
    fs::read_to_string("tests/data/ab.xml").expect("something went wrong reading the file");
  let loaded: RootElem = yaserde::de::from_str(&content).unwrap();
  println!("{:?}", &loaded);
  let yaserde_conf = yaserde::ser::Config {
    indent_string: Some(String::from("  ")),
    perform_indent: true,
    write_document_declaration: false,
  };
  let result = yaserde::ser::to_string_with_config(&loaded, &yaserde_conf).unwrap();
  println!("\n\nSerialized output:\n{:?}", &result);

  let reference = RootElem {
    // ab: AB::default(),
    children: vec![
      AB::A(A {
        attr: String::from("hallo 1"),
      }),
      AB::B(B {}),
      AB::A(A {
        attr: String::from("hallo 2"),
      }),
      AB::B(B {}),
      AB::A(A {
        attr: String::from("hallo 3"),
      }),
      AB::B(B {}),
      AB::B(B {}),
      AB::B(B {}),
      AB::B(B {}),
      AB::A(A {
        attr: String::from("hallo 4"),
      }),
      AB::B(B {}),
    ],
    // a_children: vec![
    //   A {
    //     attr: String::from("hallo 1"),
    //   },
    //   A {
    //     attr: String::from("hallo 2"),
    //   },
    //   A {
    //     attr: String::from("hallo 3"),
    //   },
    //   A {
    //     attr: String::from("hallo 4"),
    //   },
    // ],
    // b_children: vec![B {}, B {}, B {}, B {}, B {}, B {}, B {}],
  };
  assert_eq!(&loaded, &reference);

  assert_eq!(
    &result,
    r##"Root {
/* A|B elements */
A{attr = "hallo 1"}
B{}
A{attr = "hallo 2"}
B{}
A{attr = "hallo 3"}
B{}
B{}
B{}
B{}
A{attr = "hallo 4"}
B{}
/* only A elements */
A{attr = "hallo 1"}
A{attr = "hallo 2"}
A{attr = "hallo 3"}
A{attr = "hallo 4"}
/* only B elements */
B{}
B{}
B{}
B{}
B{}
B{}
B{}
}"##,
  )
}
