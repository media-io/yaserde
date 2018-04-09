
use proc_macro2::TokenNode::*;
use proc_macro2::Delimiter::Parenthesis;
use syn::Attribute;

#[derive(Debug, Clone)]
pub struct YaSerdeAttribute {
  pub root: Option<String>,
  pub rename: Option<String>,
  pub attribute: bool,
  pub text: bool,
}

impl YaSerdeAttribute {
  pub fn parse(attrs: &Vec<Attribute>) -> YaSerdeAttribute {

    let mut root = None;
    let mut rename = None;
    let mut attribute = false;
    let mut text = false;

    for attr in attrs.iter() {
      let mut attr_iter = attr.clone().tts.into_iter();
      match attr_iter.next() {
        Some(token) => {
          match token.kind {
            Group(Parenthesis, token_stream) => {
              let mut attr_iter = token_stream.into_iter();

              while let Some(item) = attr_iter.next() {
                match item.kind {
                  Term(t) => {
                    match t.as_str() {
                      "root" => {
                        attr_iter.next();
                        let v = attr_iter.next().map(|s|
                          match s.kind {
                            Literal(l) => {
                              Some(l.to_string().replace("\"", ""))
                            },
                            _ => None
                          });
                        root = v.unwrap_or(None);
                      },
                      "rename" => {
                        attr_iter.next();
                        let v = attr_iter.next().map(|s|
                          match s.kind {
                            Literal(l) => {
                              Some(l.to_string().replace("\"", ""))
                            },
                            _ => None
                          });
                        rename = v.unwrap_or(None);
                      },
                      "attribute" => {
                        attribute = true;
                      }
                      "text" => {
                        text = true;
                      }
                      _ => {},
                    }
                  },
                  _ => {}
                }
              }
            },
            _ => {},
          }
        },
        None => {},
      }
    }

    YaSerdeAttribute {
      root: root,
      rename: rename,
      attribute: attribute,
      text: text,
    }
  }
}
