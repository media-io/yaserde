pub mod build_default_value;
pub mod expand_enum;
pub mod expand_struct;

use attribute;
use proc_macro2::{Span, TokenStream};
use syn;
use syn::Ident;

pub fn expand_derive_deserialize(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
  let name = &ast.ident;
  let attrs = &ast.attrs;
  let data = &ast.data;

  let root_attrs = attribute::YaSerdeAttribute::parse(attrs);
  let root = root_attrs.clone().root.unwrap_or_else(|| name.to_string());

  let impl_block = match *data {
    syn::Data::Struct(ref data_struct) => expand_struct::parse(
      data_struct,
      name,
      &root,
      &root_attrs.prefix,
      root_attrs.strict,
      &root_attrs.namespaces,
    ),
    syn::Data::Enum(ref data_enum) => {
      expand_enum::parse(data_enum, name, &root, &root_attrs.namespaces)
    }
    syn::Data::Union(ref _data_union) => unimplemented!(),
  };

  let dummy_const = Ident::new(
    &format!("_IMPL_YA_DESERIALIZE_FOR_{}", name),
    Span::call_site(),
  );

  let generated = quote! {
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const #dummy_const: () = {
      extern crate yaserde as _yaserde;
      #impl_block
    };
  };

  Ok(generated)
}
