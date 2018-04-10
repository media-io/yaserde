
use syn;
use syn::punctuated::Pair;
use syn::Type::Path;

#[derive(Debug)]
pub enum FieldType {
  FieldTypeString,
  FieldTypeVec{data_type: syn::Ident},
  FieldTypeStruct{struct_name: syn::Ident},
}

pub fn get_field_type(field: &syn::Field) -> Option<FieldType> {
  match field.ty {
    Path(ref path) => {
      match path.path.segments.first() {
        Some(Pair::End(t)) => {
          match t.ident.to_string().as_str() {
            "String" => Some(FieldType::FieldTypeString),
            "Vec" => {
              match get_vec_type(t) {
                Some(data_type) =>
                  Some(FieldType::FieldTypeVec{
                    data_type: data_type
                  }),
                None => None,
              }
            },
            _struct_name =>
              Some(FieldType::FieldTypeStruct{
                struct_name: t.ident
              }),
          }
        },
        _ => {
          None
        },
      }
    },
    _ => {None},
  }
}

fn get_vec_type(t: &syn::PathSegment) -> Option<syn::Ident> {
  match t.arguments {
    syn::PathArguments::AngleBracketed(ref args) => {
      match args.args.first() {
        Some(Pair::End(tt)) => {
          match tt {
            &syn::GenericArgument::Type(ref argument) => {
              match argument {
                &Path(ref path2) => {
                  match path2.path.segments.first() {
                    Some(Pair::End(ttt)) => {
                      Some(ttt.ident)
                    },
                    _ => None
                  }
                },
                _ => None
              }
            },
            _ => None
          }
        },
        _ => None
      }
    },
    _ => None
  }
}
