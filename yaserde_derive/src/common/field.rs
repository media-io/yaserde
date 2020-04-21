use crate::common::attribute::YaSerdeAttribute;
use proc_macro2::{Ident, TokenStream};
use std::fmt;
use syn;
use syn::Type::Path;

#[derive(Debug)]
pub enum Field {
  FieldString,
  FieldBool,
  FieldI8,
  FieldU8,
  FieldI16,
  FieldU16,
  FieldI32,
  FieldU32,
  FieldI64,
  FieldU64,
  FieldF32,
  FieldF64,
  FieldOption { data_type: Box<Field> },
  FieldVec { data_type: Box<Field> },
  FieldStruct { struct_name: syn::Path },
}

impl Field {
  pub fn is_attribute(token_field: &syn::Field) -> bool {
    YaSerdeAttribute::parse(&token_field.attrs).attribute
  }

  pub fn is_text_content(token_field: &syn::Field) -> bool {
    YaSerdeAttribute::parse(&token_field.attrs).text
  }

  pub fn label(token_field: &syn::Field) -> Option<Ident> {
    token_field.ident.clone()
  }

  pub fn renamed_label(token_field: &syn::Field, root_attributes: &YaSerdeAttribute) -> String {
    let attributes = YaSerdeAttribute::parse(&token_field.attrs);

    let prefix = if root_attributes.default_namespace == attributes.prefix {
      "".to_string()
    } else {
      attributes
        .prefix
        .clone()
        .map_or("".to_string(), |prefix| prefix + ":")
    };

    let label = attributes
      .rename
      .clone()
      .unwrap_or_else(|| token_field.ident.as_ref().unwrap().to_string());

    format!("{}{}", prefix, label)
  }

  pub fn get_simple_type_visitor(&self) -> TokenStream {
    let ident = format_ident!("visit_{}", self.to_string());
    quote! {#ident}
  }
}

impl From<&syn::Path> for Field {
  fn from(path: &syn::Path) -> Self {
    let result = if let Some(segment) = path.segments.last() {
      match segment.ident.to_string().as_str() {
        "String" => Some(Field::FieldString),
        "bool" => Some(Field::FieldBool),
        "i8" => Some(Field::FieldI8),
        "u8" => Some(Field::FieldU8),
        "i16" => Some(Field::FieldI16),
        "u16" => Some(Field::FieldU16),
        "i32" => Some(Field::FieldI32),
        "u32" => Some(Field::FieldU32),
        "i64" => Some(Field::FieldI64),
        "u64" => Some(Field::FieldU64),
        "f32" => Some(Field::FieldF32),
        "f64" => Some(Field::FieldF64),
        "Option" => Some(Field::FieldOption {
          data_type: Box::new(Field::from(segment)),
        }),
        "Vec" => Some(Field::FieldVec {
          data_type: Box::new(Field::from(segment)),
        }),
        _ => None,
      }
    } else {
      None
    };

    result.unwrap_or_else(|| Field::FieldStruct {
      struct_name: path.clone(),
    })
  }
}

impl From<&syn::Field> for Field {
  fn from(field: &syn::Field) -> Self {
    match field.ty {
      Path(ref path) => Field::from(&path.path),
      _ => panic!("unable to match {:?}", field.ty),
    }
  }
}

impl From<&syn::PathSegment> for Field {
  fn from(path_segment: &syn::PathSegment) -> Self {
    if let syn::PathArguments::AngleBracketed(ref args) = path_segment.arguments {
      if let Some(tt) = args.args.first() {
        if let syn::GenericArgument::Type(ref argument) = *tt {
          if let Path(ref path) = *argument {
            return Field::from(&path.path);
          }
        }
      }
    }
    unimplemented!()
  }
}

impl Into<proc_macro2::TokenStream> for Field {
  fn into(self) -> proc_macro2::TokenStream {
    match self {
      Field::FieldString => quote! {String},
      Field::FieldBool => quote! {bool},
      Field::FieldI8 => quote! {i8},
      Field::FieldU8 => quote! {u8},
      Field::FieldI16 => quote! {i16},
      Field::FieldU16 => quote! {u16},
      Field::FieldI32 => quote! {i32},
      Field::FieldU32 => quote! {u32},
      Field::FieldI64 => quote! {i64},
      Field::FieldU64 => quote! {u64},
      Field::FieldF32 => quote! {f32},
      Field::FieldF64 => quote! {f64},
      _ => panic!("Not a simple type: {:?}", self),
    }
  }
}

impl Into<String> for &Field {
  fn into(self) -> String {
    match self {
      Field::FieldString => "str".to_string(),
      Field::FieldBool => "bool".to_string(),
      Field::FieldI8 => "i8".to_string(),
      Field::FieldU8 => "u8".to_string(),
      Field::FieldI16 => "i16".to_string(),
      Field::FieldU16 => "u16".to_string(),
      Field::FieldI32 => "i32".to_string(),
      Field::FieldU32 => "u32".to_string(),
      Field::FieldI64 => "i64".to_string(),
      Field::FieldU64 => "u64".to_string(),
      Field::FieldF32 => "f32".to_string(),
      Field::FieldF64 => "f64".to_string(),
      _ => panic!("Not a simple type: {:?}", self),
    }
  }
}

impl fmt::Display for Field {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string_representation: String = self.into();
    write!(f, "{}", string_representation)
  }
}
