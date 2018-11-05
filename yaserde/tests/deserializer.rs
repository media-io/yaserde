#[macro_use]
extern crate log;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::io::Read;
use yaserde::de::from_str;
use yaserde::YaDeserialize;

macro_rules! convert_and_validate {
  ($content: expr, $struct: tt, $model: expr) => {
    let loaded: Result<$struct, String> = from_str($content);
    assert_eq!(loaded, Ok($model));
  };
}

#[test]
fn de_basic() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "book")]
  pub struct Book {
    author: String,
    title: String,
  }

  let content =
    "<book><author>Antoine de Saint-Exupéry</author><title>Little prince</title></book>";
  convert_and_validate!(
    content,
    Book,
    Book {
      author: String::from("Antoine de Saint-Exupéry"),
      title: String::from("Little prince"),
    }
  );

  let content =
    "<book><title>Little prince</title><author>Antoine de Saint-Exupéry</author></book>";
  convert_and_validate!(
    content,
    Book,
    Book {
      author: String::from("Antoine de Saint-Exupéry"),
      title: String::from("Little prince"),
    }
  );
}

#[test]
fn de_list_of_items() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "library")]
  pub struct Library {
    books: Vec<String>,
  }

  let content = "<library><books>Little Prince</books><books>Harry Potter</books></library>";
  convert_and_validate!(
    content,
    Library,
    Library {
      books: vec![String::from("Little Prince"), String::from("Harry Potter")],
    }
  );

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "libraries")]
  pub struct Libraries {
    library: Vec<Library>,
  }

  let content = "<libraries><library><books>Little Prince</books></library><library><books>Harry Potter</books></library></libraries>";
  convert_and_validate!(
    content,
    Libraries,
    Libraries {
      library: vec![
        Library {
          books: vec![String::from("Little Prince")],
        },
        Library {
          books: vec![String::from("Harry Potter")],
        },
      ],
    }
  );
}

#[test]
fn de_attributes() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute)]
    item: String,
    sub: SubStruct,
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "sub")]
  pub struct SubStruct {
    #[yaserde(attribute)]
    subitem: String,
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct {
        subitem: "".to_string(),
      }
    }
  }

  let content = "<base item=\"something\"><sub subitem=\"sub-something\"></sub></base>";
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      item: "something".to_string(),
      sub: SubStruct {
        subitem: "sub-something".to_string(),
      },
    }
  );
}

#[test]
fn de_rename() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute, rename = "Item")]
    item: String,
    #[yaserde(rename = "sub")]
    sub_struct: SubStruct,
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "sub")]
  pub struct SubStruct {
    #[yaserde(attribute, rename = "sub_item")]
    subitem: String,
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct {
        subitem: "".to_string(),
      }
    }
  }

  let content = "<base Item=\"something\"><sub sub_item=\"sub_something\"></sub></base>";
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      item: "something".to_string(),
      sub_struct: SubStruct {
        subitem: "sub_something".to_string(),
      },
    }
  );
}

#[test]
fn de_text_content_with_attributes() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute, rename = "Item")]
    item: String,
    #[yaserde(rename = "sub")]
    sub_struct: SubStruct,
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "sub")]
  pub struct SubStruct {
    #[yaserde(attribute, rename = "sub_item")]
    subitem: String,
    #[yaserde(text)]
    text: String,
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct {
        subitem: "".to_string(),
        text: "".to_string(),
      }
    }
  }

  let content =
    "<base Item=\"something\"><sub sub_item=\"sub_something\">text_content</sub></base>";
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      item: "something".to_string(),
      sub_struct: SubStruct {
        subitem: "sub_something".to_string(),
        text: "text_content".to_string(),
      },
    }
  );
}

#[test]
fn de_enum() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    background: Color,
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct Colors {
    items: Vec<Color>,
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "color")]
  pub enum Color {
    White,
    Black,
  }

  impl Default for Color {
    fn default() -> Color {
      Color::White
    }
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  pub struct RGBColor {
    red: String,
    green: String,
    blue: String,
  }

  impl Default for RGBColor {
    fn default() -> RGBColor {
      RGBColor {
        red: "0".to_string(),
        green: "0".to_string(),
        blue: "0".to_string(),
      }
    }
  }

  let content =
    "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><background>Black</background></base>";
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Black,
    }
  );

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><items>Black</items><items>White</items></base>";
  convert_and_validate!(
    content,
    Colors,
    Colors {
      items: vec![Color::Black, Color::White],
    }
  );
}

#[test]
fn de_attribute_enum() {
  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute)]
    background: Color,
  }

  #[derive(YaDeserialize, PartialEq, Debug)]
  #[yaserde(root = "color")]
  pub enum Color {
    White,
    Black,
  }

  impl Default for Color {
    fn default() -> Color {
      Color::White
    }
  }

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base background=\"Black\" />";
  convert_and_validate!(
    content,
    XmlStruct,
    XmlStruct {
      background: Color::Black,
    }
  );
}
