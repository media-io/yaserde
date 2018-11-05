#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::io::Read;
use yaserde::de::from_str;
use yaserde::YaDeserialize;

#[test]
fn de_no_content() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "book")]
  pub struct Book {
    author: String,
    title: String,
  }

  let content = "";
  let loaded: Result<Book, String> = from_str(content);
  assert_eq!(
    loaded,
    Err(String::from(
      "Unexpected end of stream: no root element found"
    ))
  );
}

#[test]
fn de_wrong_end_balise() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "book")]
  pub struct Book {
    author: String,
    title: String,
  }

  let content = "<book><author>Antoine de Saint-Exupéry<title>Little prince</title></book>";
  let loaded: Result<Book, String> = from_str(content);
  assert_eq!(
    loaded,
    Err(String::from(
      "Unexpected closing tag: book, expected author"
    ))
  );
}
