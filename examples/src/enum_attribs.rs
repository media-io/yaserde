#![allow(unused)]

use yaserde::YaSerialize;

#[derive(Default, YaSerialize)]
struct A {
  #[yaserde(attribute)]
  val: u8,
}
#[derive(Default, YaSerialize)]
struct B {
  val: u8,
}
#[derive(YaSerialize)]
#[yaserde(flatten)]
enum MainNode {
#[yaserde(flatten)]
A(A),
#[yaserde(flatten)]
  B(B),
}

#[derive(Default, YaSerialize)]
struct Main {
  #[yaserde(child, flatten)]
  m: Vec<MainNode>,
}

#[cfg(test)]
mod tests {
  use super::*;

  const XML: &str =
    r##"<?xml version="1.0" encoding="utf-8"?><Main><A val="1" /><B><val>2</val></B></Main>"##;
  #[test]
  fn attribute() {
    let node = Main {
      m: vec![MainNode::A(A { val: 1 }), MainNode::B(B { val: 2 })],
    };
    let data = yaserde::ser::to_string(&node).unwrap();
    assert_eq!(data, XML);
  }
}
