use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use serde::Deserialize;
use serde_tokenstream::from_tokenstream;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use syn::{Attribute, Meta};

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct YaSerdeAttribute {
  /// Set this field as an XML attribute
  #[serde(default)]
  pub attribute: bool,
  /// Set default callback function
  #[serde(default)]
  pub default: Option<String>,
  /// Set the default namespace
  #[serde(default)]
  pub default_namespace: Option<String>,
  /// Flatten child fields
  #[serde(default)]
  pub flatten: bool,
  /// Declare all namespaces with prefix/URL
  #[serde(default)]
  pub namespaces: BTreeMap<String, String>,
  /// Set the prefix for the scope
  #[serde(default)]
  pub prefix: Option<String>,
  /// Rename the field/struct/enum name
  #[serde(default)]
  pub rename: Option<String>,
  #[serde(default)]
  pub tag: Option<String>,
  /// Disable the serialization for the field
  #[serde(default)]
  pub skip_serializing: bool,
  /// Disable the serialization for the field based on a condition
  #[serde(default)]
  pub skip_serializing_if: Option<String>,
  /// Set the field as an XML text content
  #[serde(default)]
  pub text: bool,
  /// Set the field as an XML CDATA content
  #[serde(default)]
  pub cdata: bool,
}

impl TryFrom<&Attribute> for YaSerdeAttribute {
  type Error = String;

  fn try_from(attr: &Attribute) -> Result<Self, Self::Error> {
    if attr.path().is_ident("yaserde") {
      let attributes = match &attr.meta {
        Meta::Path(_) => {
          unreachable!()
        }
        Meta::List(list) => {
          let mut tokens = TokenStream::new();
          list.tokens.to_tokens(&mut tokens);

          match from_tokenstream::<YaSerdeAttribute>(&tokens) {
            Ok(attribute) => attribute,
            Err(error) => {
              panic!("YaSerDe derive error: {}", error);
            }
          }
        }
        Meta::NameValue(_) => {
          unreachable!()
        }
      };

      Ok(attributes)
    } else {
      Err("not an attribute".to_string())
    }
  }
}

impl From<&Vec<Attribute>> for YaSerdeAttribute {
  fn from(attributes: &Vec<Attribute>) -> Self {
    attributes
      .iter()
      .find_map(|attribute| YaSerdeAttribute::try_from(attribute).ok())
      .unwrap_or_default()
  }
}

impl YaSerdeAttribute {
  pub fn xml_element_name(&self, ident: &Ident) -> String {
    self.rename.clone().unwrap_or_else(|| ident.to_string())
  }

  pub fn prefix_namespace(&self) -> String {
    if self.default_namespace == self.prefix {
      "".to_string()
    } else {
      self
        .clone()
        .prefix
        .map_or("".to_string(), |prefix| prefix + ":")
    }
  }

  pub fn get_namespace_matching(
    &self,
    prefix: &Option<String>,
    element_namespace: TokenStream,
    element_name: TokenStream,
    take_root_prefix: bool,
  ) -> TokenStream {
    let configured_prefix = if take_root_prefix {
      self.prefix.clone()
    } else {
      prefix.clone()
    };

    let namespaces_matches: TokenStream = self
      .namespaces
      .iter()
      .filter_map(|(prefix, namespace)| {
        if configured_prefix.as_deref().eq(&Some(prefix)) {
          Some(quote!(#namespace => {}))
        } else {
          None
        }
      })
      .collect();

    quote!(
      if let Some(namespace) = #element_namespace {
        match namespace.as_str() {
          #namespaces_matches
          bad_namespace => {
            let msg =
              ::std::format!("bad namespace for {}, found {}", #element_name, bad_namespace);
            return Err(msg);
          }
        }
      }
    )
  }
}
