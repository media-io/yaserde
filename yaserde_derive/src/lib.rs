#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

mod der;

use proc_macro::TokenStream;

fn expand_derive_serialize(ast: &syn::DeriveInput) -> quote::Tokens {
  let name = &ast.ident;
  quote! {
    impl YaSerialize for #name {
      fn derive_serialize() {
        println!("serialize {}", stringify!(#name));
      }
    }
  }
}

#[proc_macro_derive(YaDeserialize, attributes(yaserde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  match der::expand_derive_deserialize(&ast) {
    Ok(expanded) => expanded.into(),
    Err(msg) => panic!(msg),
  }
}

#[proc_macro_derive(YaSerialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  let gen = expand_derive_serialize(&ast);
  gen.into()
}
