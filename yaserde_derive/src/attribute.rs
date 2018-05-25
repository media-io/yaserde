
use proc_macro2::TokenTree;
use proc_macro2::Delimiter;
use proc_macro2::token_stream::IntoIter;
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

fn get_value(iter: &mut IntoIter) -> Option<String> {
  if let (Some(TokenTree::Punct(operator)), Some(TokenTree::Literal(value))) = (iter.next(), iter.next()) {
    if operator.as_char() == '=' {
      Some(value.to_string().replace("\"", ""))
    } else {
      None
    }
  } else {
    None
  }
}

impl YaSerdeAttribute {
  pub fn parse(attrs: &[Attribute]) -> YaSerdeAttribute {
    let mut attribute = false;
    let mut namespaces = BTreeMap::new();
    let mut prefix = None;
    let mut rename = None;
    let mut root = None;
    let mut text = false;

    for attr in attrs.iter() {
      let mut attr_iter = attr.clone().tts.into_iter();
      if let Some(token) = attr_iter.next() {
        if let TokenTree::Group(group) = token {
          if group.delimiter() == Delimiter::Parenthesis {
            let mut attr_iter = group.stream().into_iter();

            while let Some(item) = attr_iter.next() {
              if let TokenTree::Ident(ident) = item {
                match ident.to_string().as_str() {
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
                }
              }
            }
          }
        }
      }
    }

    YaSerdeAttribute {
      attribute,
      namespaces,
      prefix,
      rename,
      root,
      text,
    }
  }
}
