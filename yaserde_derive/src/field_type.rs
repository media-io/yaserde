use syn;
use syn::Type::Path;

#[derive(Debug)]
pub enum FieldType {
  FieldTypeString,
  FieldTypeBool,
  FieldTypeI8,
  FieldTypeU8,
  FieldTypeI16,
  FieldTypeU16,
  FieldTypeI32,
  FieldTypeU32,
  FieldTypeI64,
  FieldTypeU64,
  FieldTypeF32,
  FieldTypeF64,
  FieldTypeOption { data_type: Box<FieldType> },
  FieldTypeVec { data_type: Box<FieldType> },
  FieldTypeStruct { struct_name: syn::Path },
}

impl FieldType {
  fn from_ident(path: &syn::Path) -> Option<FieldType> {
    match path.segments.last() {
      Some(t) => match t.ident.to_string().as_str() {
        "String" => Some(FieldType::FieldTypeString),
        "bool" => Some(FieldType::FieldTypeBool),
        "i8" => Some(FieldType::FieldTypeI8),
        "u8" => Some(FieldType::FieldTypeU8),
        "i16" => Some(FieldType::FieldTypeI16),
        "u16" => Some(FieldType::FieldTypeU16),
        "i32" => Some(FieldType::FieldTypeI32),
        "u32" => Some(FieldType::FieldTypeU32),
        "i64" => Some(FieldType::FieldTypeI64),
        "u64" => Some(FieldType::FieldTypeU64),
        "f32" => Some(FieldType::FieldTypeF32),
        "f64" => Some(FieldType::FieldTypeF64),
        "Option" => get_sub_type(t).map(|data_type| FieldType::FieldTypeOption {
          data_type: Box::new(FieldType::from_ident(&syn::Path::from(data_type)).unwrap()),
        }),
        "Vec" => get_sub_type(t).map(|data_type| FieldType::FieldTypeVec {
          data_type: Box::new(FieldType::from_ident(&syn::Path::from(data_type)).unwrap()),
        }),
        _ => Some(FieldType::FieldTypeStruct {
          struct_name: path.clone(),
        }),
      },
      _ => None,
    }
  }
}

pub fn get_field_type(field: &syn::Field) -> Option<FieldType> {
  match field.ty {
    Path(ref path) => FieldType::from_ident(&path.path),
    _ => None,
  }
}

fn get_sub_type(t: &syn::PathSegment) -> Option<syn::PathSegment> {
  if let syn::PathArguments::AngleBracketed(ref args) = t.arguments {
    if let Some(tt) = args.args.first() {
      if let syn::GenericArgument::Type(ref argument) = *tt {
        if let Path(ref path2) = *argument {
          if let Some(ttt) = path2.path.segments.first() {
            return Some(ttt.clone());
          }
        }
      }
    }
  }

  None
}

pub fn get_simple_type_token(field_type: &FieldType) -> proc_macro2::TokenStream {
  match field_type {
    FieldType::FieldTypeString => quote! {String},
    FieldType::FieldTypeBool => quote! {bool},
    FieldType::FieldTypeI8 => quote! {i8},
    FieldType::FieldTypeU8 => quote! {u8},
    FieldType::FieldTypeI16 => quote! {i16},
    FieldType::FieldTypeU16 => quote! {u16},
    FieldType::FieldTypeI32 => quote! {i32},
    FieldType::FieldTypeU32 => quote! {u32},
    FieldType::FieldTypeI64 => quote! {i64},
    FieldType::FieldTypeU64 => quote! {u64},
    FieldType::FieldTypeF32 => quote! {f32},
    FieldType::FieldTypeF64 => quote! {f64},
    _ => panic!("Not a simple type: {:?}", field_type),
  }
}

pub fn get_simple_type_visitor(field_type: &FieldType) -> proc_macro2::TokenStream {
  let ident = format_ident!(
    "visit_{}",
    get_simple_type_token(field_type)
      .to_string()
      .replace("String", "str")
  );

  quote! {#ident}
}
