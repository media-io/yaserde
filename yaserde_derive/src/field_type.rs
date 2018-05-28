use syn;
use syn::punctuated::Pair;
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
  FieldTypeVec { data_type: Box<FieldType> },
  FieldTypeStruct { struct_name: syn::Ident },
}

impl FieldType {
  fn from_ident(t: &syn::PathSegment) -> Option<FieldType> {
    match t.ident.to_string().as_str() {
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
      "Vec" => get_vec_type(t).map(|data_type| {
        let p = syn::PathSegment {
          ident: data_type,
          arguments: syn::PathArguments::None,
        };

        FieldType::FieldTypeVec {
          data_type: Box::new(FieldType::from_ident(&p).unwrap()),
        }
      }),
      _struct_name => Some(FieldType::FieldTypeStruct {
        struct_name: t.ident.clone(),
      }),
    }
  }
}

pub fn get_field_type(field: &syn::Field) -> Option<FieldType> {
  match field.ty {
    Path(ref path) => match path.path.segments.first() {
      Some(Pair::End(t)) => FieldType::from_ident(t),
      _ => None,
    },
    _ => None,
  }
}

fn get_vec_type(t: &syn::PathSegment) -> Option<syn::Ident> {
  if let syn::PathArguments::AngleBracketed(ref args) = t.arguments {
    if let Some(Pair::End(tt)) = args.args.first() {
      if let syn::GenericArgument::Type(ref argument) = *tt {
        if let Path(ref path2) = *argument {
          if let Some(Pair::End(ttt)) = path2.path.segments.first() {
            return Some(ttt.ident.clone());
          }
        }
      }
    }
  }

  None
}
