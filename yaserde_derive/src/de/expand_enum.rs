use attribute::*;
use field_type::*;
use quote::Tokens;
use std::collections::BTreeMap;
use syn::Fields;
use syn::Ident;
use syn::DataEnum;
use proc_macro2::Span;

pub fn parse(
  data_enum: &DataEnum,
  name: &Ident,
  root: &str,
  _namespaces: &BTreeMap<String, String>,
) -> Tokens {
  let variables: Tokens = data_enum
    .variants
    .iter()
    .map(|ref variant| match variant.fields {
      Fields::Unit => None,
      Fields::Named(ref fields) => {
        let enum_fields = fields
          .named
          .iter()
          .map(|ref field| {
            let field_label = field.ident;

            match get_field_type(field) {
              Some(FieldType::FieldTypeString) => {
                build_default_value(&field_label, &quote!{String}, &quote!{"".to_string()})
              }
              Some(FieldType::FieldTypeBool) => {
                build_default_value(&field_label, &quote!{bool}, &quote!{false})
              }
              Some(FieldType::FieldTypeI8) => {
                build_default_value(&field_label, &quote!{i8}, &quote!{0})
              }
              Some(FieldType::FieldTypeU8) => {
                build_default_value(&field_label, &quote!{u8}, &quote!{0})
              }
              Some(FieldType::FieldTypeI16) => {
                build_default_value(&field_label, &quote!{i16}, &quote!{0})
              }
              Some(FieldType::FieldTypeU16) => {
                build_default_value(&field_label, &quote!{u16}, &quote!{0})
              }
              Some(FieldType::FieldTypeI32) => {
                build_default_value(&field_label, &quote!{i32}, &quote!{0})
              }
              Some(FieldType::FieldTypeU32) => {
                build_default_value(&field_label, &quote!{u32}, &quote!{0})
              }
              Some(FieldType::FieldTypeI64) => {
                build_default_value(&field_label, &quote!{i64}, &quote!{0})
              }
              Some(FieldType::FieldTypeU64) => {
                build_default_value(&field_label, &quote!{u64}, &quote!{0})
              }
              Some(FieldType::FieldTypeStruct { struct_name }) => Some(quote!{
                #[allow(unused_mut)]
                let mut #field_label : #struct_name = #struct_name::default();
              }),
              Some(FieldType::FieldTypeVec { data_type }) => {
                let dt = Box::into_raw(data_type);
                match unsafe { dt.as_ref() } {
                  Some(&FieldType::FieldTypeString) => {
                    build_default_value(&field_label, &quote!{Vec<String>}, &quote!{vec![]})
                  }
                  Some(&FieldType::FieldTypeBool) => {
                    build_default_value(&field_label, &quote!{Vec<bool>}, &quote!{vec![]})
                  }
                  Some(&FieldType::FieldTypeI8) => {
                    build_default_value(&field_label, &quote!{Vec<i8>}, &quote!{vec![]})
                  }
                  Some(&FieldType::FieldTypeU8) => {
                    build_default_value(&field_label, &quote!{Vec<u8>}, &quote!{vec![]})
                  }
                  Some(&FieldType::FieldTypeI16) => {
                    build_default_value(&field_label, &quote!{Vec<i16>}, &quote!{vec![]})
                  }
                  Some(&FieldType::FieldTypeU16) => {
                    build_default_value(&field_label, &quote!{Vec<u16>}, &quote!{vec![]})
                  }
                  Some(&FieldType::FieldTypeI32) => {
                    build_default_value(&field_label, &quote!{Vec<i32>}, &quote!{vec![]})
                  }
                  Some(&FieldType::FieldTypeU32) => {
                    build_default_value(&field_label, &quote!{Vec<u32>}, &quote!{vec![]})
                  }
                  Some(&FieldType::FieldTypeI64) => {
                    build_default_value(&field_label, &quote!{Vec<i64>}, &quote!{vec![]})
                  }
                  Some(&FieldType::FieldTypeU64) => {
                    build_default_value(&field_label, &quote!{Vec<u64>}, &quote!{vec![]})
                  }
                  Some(&FieldType::FieldTypeStruct { struct_name }) => Some(quote!{
                    #[allow(unused_mut)]
                    let mut #field_label : Vec<#struct_name> = vec![];
                  }),
                  Some(&FieldType::FieldTypeVec { .. }) => {
                    unimplemented!();
                  }
                  None => {
                    unimplemented!();
                  }
                }
              }
              None => None,
            }
          })
          .filter(|x| x.is_some())
          .map(|x| x.unwrap())
          .fold(Tokens::new(), |mut sum, val| {
            sum.append_all(val);
            sum
          });

        Some(enum_fields)
      }
      Fields::Unnamed(ref _fields) => {
        unimplemented!();
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {
      sum.append_all(val);
      sum
    });

  let enum_visitors: Tokens = data_enum
    .variants
    .iter()
    .map(|ref variant| {
      match variant.fields {
        Fields::Unit => None,
        Fields::Named(ref fields) => {
          let enum_fields = fields
            .named
            .iter()
            .map(|ref field| {
              // let label = field.ident;
              // let label_name = label.unwrap().to_string();
              // let visitor_label = Ident::new(&format!("__Visitor{}", label_name), Span::call_site());

              match get_field_type(field) {
                Some(FieldType::FieldTypeString) => {
                  Some(quote!{
                    // struct #visitor_label;
                    // impl<'de> Visitor<'de> for #visitor_label {
                    //   type Value = String;

                    //   fn visit_str(self, v: &str) -> Result<Self::Value, String> {
                    //     match v {
                    //       _ => Err("unable to match \"{}\" with enum {}", v, #label_name)
                    //     }
                    //     Ok(String::from(v))
                    //   }
                    // }
                  })
                }
                _ => None,
              }
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .fold(Tokens::new(), |mut sum, val| {
              sum.append_all(val);
              sum
            });

          Some(enum_fields)
        }
        Fields::Unnamed(ref _fields) => {
          unimplemented!();
        }
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {
      sum.append_all(val);
      sum
    });

  let match_to_enum: Tokens = data_enum
    .variants
    .iter()
    .map(|ref variant| {
      let field_attrs = YaSerdeAttribute::parse(&variant.attrs);
      let renamed_label = match field_attrs.rename {
        Some(value) => Ident::new(&format!("{}", value), Span::call_site()),
        None => variant.ident,
      };
      let label = variant.ident;
      let label_name = renamed_label.to_string();

      match variant.fields {
        Fields::Unit => Some(quote!{
          #label_name => {
            simple_enum_value = Some(#name::#label);
          }
        }),
        _ => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut tokens, token| {
      tokens.append_all(token);
      tokens
    });

  quote!{
    use xml::reader::XmlEvent;

    impl YaDeserialize for #name {
      #[allow(unused_variables)]
      fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        let named_element =
          if let XmlEvent::StartElement{name, ..} = reader.peek()?.to_owned() {
            name.local_name.to_owned()
          } else {
            String::from(#root)
          };
        debug!("Enum: start to parse {:?}", named_element);

        #[allow(unused_assignments, unused_mut)]
        let mut simple_enum_value = None;

        #variables
        #enum_visitors

        loop {
          match reader.peek()?.to_owned() {
            XmlEvent::StartElement{name, attributes, namespace: _namespace} => {
              debug!("Enum: {}: {}", named_element, name.local_name.as_str());
              if name.local_name == named_element {
                let _next = reader.next();

                if let XmlEvent::Characters(content) = reader.peek()?.to_owned() {
                  match content.as_str() {
                    #match_to_enum
                    _ => {}
                  }
                }
              }
            },
            XmlEvent::EndElement{name} => {
              if name.local_name.as_str() == named_element {
                break;
              }
              let _root = reader.next();
            },
            xml::reader::XmlEvent::Characters(characters_content) => {
              let _root = reader.next();
            },
            event => {
              return Err(format!("unknown event {:?}", event))
            },
          }
        }

        match simple_enum_value {
          Some(value) => Ok(value),
          None => {
            Ok(#name::default())
          },
        }
      }
    }
  }
}

fn build_default_value(
  label: &Option<Ident>,
  field_type: &Tokens,
  default: &Tokens,
) -> Option<Tokens> {
  Some(quote!{
    #[allow(unused_mut)]
    let mut #label : #field_type = #default;
  })
}
