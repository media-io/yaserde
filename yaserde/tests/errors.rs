#[macro_use]
extern crate yaserde_derive;

use yaserde::de::from_str;

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
    Err("Unexpected end of stream: no root element found".to_owned())
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
    Err("Unexpected closing tag: book, expected author".to_owned())
  );
}
