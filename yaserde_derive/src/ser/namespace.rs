use crate::attribute::YaSerdeAttribute;
use proc_macro2::TokenStream;

pub fn generate_namespaces_definition(attributes: &YaSerdeAttribute) -> TokenStream {
  attributes
    .namespaces
    .iter()
    .map(|(prefix, namespace)| {
      if let Some(dn) = &attributes.default_namespace {
        if dn == prefix {
          return Some(quote!(
            .default_ns(#namespace)
          ));
        }
      }
      Some(quote!(
        .ns(#prefix, #namespace)
      ))
    })
    .filter_map(|x| x)
    .collect()
}
