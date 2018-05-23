use proc_macro2::TokenTreeIter;
use proc_macro2::TokenNode::*;
use proc_macro2::Spacing;
use proc_macro2::Delimiter::Parenthesis;
use std::collections::BTreeMap;
use syn::Attribute;

#[derive(Debug, Clone)]
pub struct YaSerdeAttribute {
  pub root: Option<String>,
  pub rename: Option<String>,
  pub prefix: Option<String>,
  pub namespaces: BTreeMap<String, String>,
  pub attribute: bool,
  pub text: bool,
}

fn get_value(iter: &mut TokenTreeIter) -> Option<String> {
  match (iter.next(), iter.next()) {
    (Some(operator), Some(value)) => match (operator.kind, value.kind) {
      (Op('=', Spacing::Alone), Literal(l)) => Some(l.to_string().replace("\"", "")),
      _ => None,
    },
    _ => None,
  }
}

impl YaSerdeAttribute {
  pub fn parse(attrs: &Vec<Attribute>) -> YaSerdeAttribute {
    let mut attribute = false;
    let mut namespaces = BTreeMap::new();
    let mut prefix = None;
    let mut rename = None;
    let mut root = None;
    let mut text = false;

    for attr in attrs.iter() {
      let mut attr_iter = attr.clone().tts.into_iter();
      match attr_iter.next() {
        Some(token) => match token.kind {
          Group(Parenthesis, token_stream) => {
            let mut attr_iter = token_stream.into_iter();

            while let Some(item) = attr_iter.next() {
              match item.kind {
                Term(t) => match t.as_str() {
                  "attribute" => {
                    attribute = true;
                  }
                  "namespace" => {
                    if let Some(namespace) = get_value(&mut attr_iter) {
                      let splitted: Vec<&str> = namespace.split(": ").collect();
                      if splitted.len() == 2 {
                        namespaces.insert(splitted[0].to_owned(), splitted[1].to_owned());
                      }
                      if splitted.len() == 1 {
                        namespaces.insert("".to_owned(), splitted[0].to_owned());
                      }
                    }
                  }
                  "prefix" => {
                    prefix = get_value(&mut attr_iter);
                  }
                  "rename" => {
                    rename = get_value(&mut attr_iter);
                  }
                  "root" => {
                    root = get_value(&mut attr_iter);
                  }
                  "text" => {
                    text = true;
                  }
                  _ => {}
                },
                _ => {}
              }
            }
          }
          _ => {}
        },
        None => {}
      }
    }

    YaSerdeAttribute {
      attribute: attribute,
      namespaces: namespaces,
      prefix: prefix,
      rename: rename,
      root: root,
      text: text,
    }
  }
}
