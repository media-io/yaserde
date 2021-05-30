#![recursion_limit = "256"]

// Required for Rust < 1.42
extern crate proc_macro;

mod common;
mod de;
mod ser;

use proc_macro::TokenStream;

#[proc_macro_derive(YaDeserialize, attributes(yaserde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  match de::expand_derive_deserialize(&ast) {
    Ok(expanded) => expanded.into(),
    Err(msg) => panic!("{}", msg),
  }
}

#[proc_macro_derive(YaSerialize, attributes(yaserde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  match ser::expand_derive_serialize(&ast) {
    Ok(expanded) => expanded.into(),
    Err(msg) => panic!("{}", msg),
  }
}
