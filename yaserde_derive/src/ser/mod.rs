pub mod element;
pub mod expand_enum;
pub mod expand_struct;
pub mod implement_deserializer;
pub mod label;
pub mod namespace;

use crate::common::YaSerdeAttribute;
use proc_macro2::TokenStream;
use syn;
use syn::Ident;

pub fn expand_derive_serialize(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
  let name = &ast.ident;
  let attrs = &ast.attrs;
  let data = &ast.data;

  let root_attributes = YaSerdeAttribute::parse(attrs);
  let root_name = root_attributes
    .clone()
    .rename
    .unwrap_or_else(|| name.to_string());

  let prefix = if root_attributes.default_namespace == root_attributes.prefix {
    "".to_string()
  } else {
    root_attributes
      .clone()
      .prefix
      .map_or("".to_string(), |prefix| prefix + ":")
  };

  let root_name = format!("{}{}", prefix, root_name);

  let impl_block = match *data {
    syn::Data::Struct(ref data_struct) => {
      expand_struct::serialize(data_struct, name, &root_name, &root_attributes)
    }
    syn::Data::Enum(ref data_enum) => {
      expand_enum::serialize(data_enum, name, &root_name, &root_attributes)
    }
    syn::Data::Union(ref _data_union) => unimplemented!(),
  };

  let dummy_const = Ident::new(&format!("_IMPL_YA_SERIALIZE_FOR_{}", name), name.span());

  let generated = quote! {
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const #dummy_const: () = {
      extern crate yaserde as _yaserde;
      #impl_block
    };
  };

  Ok(generated)
}
