use proc_macro2::{Span, TokenStream};
use syn::Ident;

pub fn build_default_value(
  label: &Option<Ident>,
  field_type: &TokenStream,
  value: &TokenStream,
  default: &Option<String>,
) -> Option<TokenStream> {
  let value = default
    .as_ref()
    .map(|d| {
      let default_function = Ident::new(
        &d,
        label
          .as_ref()
          .map_or(Span::call_site(), |ident| ident.span()),
      );

      quote!(#default_function())
    })
    .unwrap_or_else(|| quote!(#value));

  Some(quote! {
    #[allow(unused_mut)]
    let mut #label : #field_type = #value;
  })
}
