use crate::common::YaSerdeAttribute;
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_namespaces_definition(attributes: &YaSerdeAttribute) -> TokenStream {
  attributes
    .namespaces
    .iter()
    .map(|(prefix, namespace)| {
      if attributes.default_namespace.eq(prefix) {
          return Some(quote!(
            .default_ns(#namespace)
          ));
      }
      Some(quote!(
        .ns(#prefix, #namespace)
      ))
    })
    .flatten()
    .collect()
}
