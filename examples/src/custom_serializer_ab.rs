use yaserde::{YaDeserialize, YaSerialize};

#[derive(Debug, YaDeserialize)]
struct RootElem {
  #[yaserde(child)]
  children: Vec<AB>,
  // children are filtered by A and B
  // this is usually not desired
  #[yaserde(rename = "A")]
  a_children: Vec<A>,
  #[yaserde(rename = "B")]
  b_children: Vec<B>,
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
    writer.write("/* only A elements */\n").unwrap();
    for e in &self.a_children {
      e.serialize(writer).unwrap();
    }
    writer.write("/* only B elements */\n").unwrap();
    for e in &self.b_children {
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

#[derive(Debug, YaDeserialize)]
struct A {
  #[yaserde(attribute)]
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

#[derive(Debug, YaDeserialize)]
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

#[derive(Debug, Default, YaDeserialize)]
enum AB {
  #[default]
  None,
  #[yaserde(rename = "A")]
  A(A),
  #[yaserde(rename = "B")]
  B(B),
}
impl YaSerialize for AB {
  fn serialize<W: std::io::prelude::Write>(
    &self,
    writer: &mut yaserde::ser::Serializer<W>,
  ) -> Result<(), String> {
    writer.write("/* serialized AB */\n").unwrap();
    match self {
      Self::None => (),
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
  assert_eq!(
    &result,
    r##"Root {
/* A|B elements */
A{}
B{}
A{}
B{}
A{}
B{}
B{}
B{}
B{}
A{}
B{}
/* only A elements */
A{}
A{}
A{}
A{}
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
