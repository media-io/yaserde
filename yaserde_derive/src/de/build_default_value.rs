use proc_macro2::{Span, TokenStream};
use syn::Ident;

pub fn build_default_value(
  label: &Option<Ident>,
  field_type: &TokenStream,
  value: &TokenStream,
  default: &Option<String>,
) -> Option<TokenStream> {
  if let Some(d) = default {
    let default_function = Ident::new(&d, Span::call_site());

    Some(quote! {
      #[allow(unused_mut)]
      let mut #label : #field_type = #default_function();
    })
  } else {
    Some(quote! {
      #[allow(unused_mut)]
      let mut #label : #field_type = #value;
    })
  }
}
