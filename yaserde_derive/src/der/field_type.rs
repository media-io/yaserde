
use syn;
use syn::punctuated::Pair;
use syn::Type::Path;

#[derive(Debug)]
pub enum FieldType {
  FieldTypeString,
  FieldTypeVec,
  FieldTypeStruct{name: String},
}

pub fn get_field_type(field: &syn::Field) -> Option<FieldType> {
  match field.ty {
    Path(ref path) => {
      match path.path.segments.first() {
        Some(Pair::End(t)) => {
          match t.ident.to_string().as_str() {
            "String" => Some(FieldType::FieldTypeString),
            "Vec" => Some(FieldType::FieldTypeVec),
            name => Some(FieldType::FieldTypeStruct{name: name.to_string()}),
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

pub fn get_vec_type(field: &syn::Field) -> Option<syn::Ident> {
  match field.ty {
    Path(ref path) => {
      match path.path.segments.first() {
        Some(Pair::End(t)) => {
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
        },
        _ => None
      }
    }
    _ => None
  }
}