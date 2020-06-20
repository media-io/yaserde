#[macro_use]
extern crate yaserde_derive;

use std::io::Read;
use yaserde::de::from_str;
use yaserde::YaDeserialize;

fn init() {
  let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn de_no_content() {
  init();

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
  init();

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
