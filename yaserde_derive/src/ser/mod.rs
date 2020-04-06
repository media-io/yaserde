pub mod element;
pub mod expand_enum;
pub mod expand_struct;

use attribute;
use proc_macro2::TokenStream;
use syn;
use syn::Ident;

pub fn expand_derive_serialize(ast: &syn::DeriveInput) -> Result<TokenStream, String> {
  let name = &ast.ident;
  let attrs = &ast.attrs;
  let data = &ast.data;

  let root_attrs = attribute::YaSerdeAttribute::parse(attrs);
  let root = root_attrs.clone().root.unwrap_or_else(|| name.to_string());

  let prefix =
    if root_attrs.default_namespace == root_attrs.prefix {
      "".to_string()
    } else {
      root_attrs.prefix.map_or("".to_string(), |prefix| prefix + ":")
    };

  let root = format!("{}{}", prefix, root);

  let impl_block = match *data {
    syn::Data::Struct(ref data_struct) => expand_struct::serialize(
      data_struct,
      name,
      &root,
      &root_attrs.namespaces,
      &root_attrs.default_namespace,
    ),
    syn::Data::Enum(ref data_enum) => expand_enum::serialize(
      data_enum,
      name,
      &root,
      &root_attrs.namespaces,
      &root_attrs.default_namespace,
    ),
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
