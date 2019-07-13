use attribute::*;
use de::build_default_value::build_default_value;
use field_type::*;
use proc_macro2::{Span, TokenStream};
use quote::TokenStreamExt;
use std::collections::BTreeMap;
use syn::DataEnum;
use syn::Fields;
use syn::Ident;

pub fn parse(
  data_enum: &DataEnum,
  name: &Ident,
  root: &str,
  _namespaces: &BTreeMap<String, String>,
) -> TokenStream {
  let variables: TokenStream = data_enum
    .variants
    .iter()
    .map(|variant| match variant.fields {
      Fields::Unit => None,
      Fields::Named(ref fields) => {
        let enum_fields = fields
          .named
          .iter()
          .map(|field| {
            let field_label = &field.ident;
            let field_attrs = YaSerdeAttribute::parse(&field.attrs);

            match get_field_type(field) {
              Some(FieldType::FieldTypeString) => build_default_value(
                field_label,
                &quote! {String},
                &quote! {"".to_string()},
                &field_attrs.default,
              ),
              Some(FieldType::FieldTypeBool) => build_default_value(
                field_label,
                &quote! {bool},
                &quote! {false},
                &field_attrs.default,
              ),
              Some(FieldType::FieldTypeI8) => {
                build_default_value(field_label, &quote! {i8}, &quote! {0}, &field_attrs.default)
              }
              Some(FieldType::FieldTypeU8) => {
                build_default_value(field_label, &quote! {u8}, &quote! {0}, &field_attrs.default)
              }
              Some(FieldType::FieldTypeI16) => build_default_value(
                field_label,
                &quote! {i16},
                &quote! {0},
                &field_attrs.default,
              ),
              Some(FieldType::FieldTypeU16) => build_default_value(
                field_label,
                &quote! {u16},
                &quote! {0},
                &field_attrs.default,
              ),
              Some(FieldType::FieldTypeI32) => build_default_value(
                field_label,
                &quote! {i32},
                &quote! {0},
                &field_attrs.default,
              ),
              Some(FieldType::FieldTypeU32) => build_default_value(
                field_label,
                &quote! {u32},
                &quote! {0},
                &field_attrs.default,
              ),
              Some(FieldType::FieldTypeI64) => build_default_value(
                field_label,
                &quote! {i64},
                &quote! {0},
                &field_attrs.default,
              ),
              Some(FieldType::FieldTypeU64) => build_default_value(
                field_label,
                &quote! {u64},
                &quote! {0},
                &field_attrs.default,
              ),
              Some(FieldType::FieldTypeF32) => build_default_value(
                field_label,
                &quote! {f32},
                &quote! {0},
                &field_attrs.default,
              ),
              Some(FieldType::FieldTypeF64) => build_default_value(
                field_label,
                &quote! {f64},
                &quote! {0},
                &field_attrs.default,
              ),
              Some(FieldType::FieldTypeStruct { struct_name }) => build_default_value(
                field_label,
                &quote! {#struct_name},
                &quote! {#struct_name::default()},
                &field_attrs.default,
              ),
              Some(FieldType::FieldTypeOption { .. }) => {
                if let Some(d) = &field_attrs.default {
                  let default_function = Ident::new(&d, Span::call_site());

                  Some(quote! {
                    #[allow(unused_mut, non_snake_case, non_camel_case_types)]
                    let mut #field_label = #default_function();
                  })
                } else {
                  Some(quote! {
                    #[allow(unused_mut, non_snake_case, non_camel_case_types)]
                    let mut #field_label = None;
                  })
                }
              }
              Some(FieldType::FieldTypeVec { data_type }) => {
                let dt = Box::into_raw(data_type);
                match unsafe { dt.as_ref() } {
                  Some(&FieldType::FieldTypeString) => build_default_value(
                    field_label,
                    &quote! {Vec<String>},
                    &quote! {vec![]},
                    &field_attrs.default,
                  ),
                  Some(&FieldType::FieldTypeBool) => build_default_value(
                    field_label,
                    &quote! {Vec<bool>},
                    &quote! {vec![]},
                    &field_attrs.default,
                  ),
                  Some(&FieldType::FieldTypeI8) => build_default_value(
                    field_label,
                    &quote! {Vec<i8>},
                    &quote! {vec![]},
                    &field_attrs.default,
                  ),
                  Some(&FieldType::FieldTypeU8) => build_default_value(
                    field_label,
                    &quote! {Vec<u8>},
                    &quote! {vec![]},
                    &field_attrs.default,
                  ),
                  Some(&FieldType::FieldTypeI16) => build_default_value(
                    field_label,
                    &quote! {Vec<i16>},
                    &quote! {vec![]},
                    &field_attrs.default,
                  ),
                  Some(&FieldType::FieldTypeU16) => build_default_value(
                    field_label,
                    &quote! {Vec<u16>},
                    &quote! {vec![]},
                    &field_attrs.default,
                  ),
                  Some(&FieldType::FieldTypeI32) => build_default_value(
                    field_label,
                    &quote! {Vec<i32>},
                    &quote! {vec![]},
                    &field_attrs.default,
                  ),
                  Some(&FieldType::FieldTypeU32) => build_default_value(
                    field_label,
                    &quote! {Vec<u32>},
                    &quote! {vec![]},
                    &field_attrs.default,
                  ),
                  Some(&FieldType::FieldTypeI64) => build_default_value(
                    field_label,
                    &quote! {Vec<i64>},
                    &quote! {vec![]},
                    &field_attrs.default,
                  ),
                  Some(&FieldType::FieldTypeU64) => build_default_value(
                    field_label,
                    &quote! {Vec<u64>},
                    &quote! {vec![]},
                    &field_attrs.default,
                  ),
                  Some(&FieldType::FieldTypeF32) => build_default_value(
                    field_label,
                    &quote! {Vec<f32>},
                    &quote! {vec![]},
                    &field_attrs.default,
                  ),
                  Some(&FieldType::FieldTypeF64) => build_default_value(
                    field_label,
                    &quote! {Vec<f64>},
                    &quote! {vec![]},
                    &field_attrs.default,
                  ),
                  Some(&FieldType::FieldTypeStruct { ref struct_name }) => build_default_value(
                    field_label,
                    &quote! {Vec<#struct_name>},
                    &quote! {vec![]},
                    &field_attrs.default,
                  ),
                  Some(&FieldType::FieldTypeOption { .. })
                  | Some(&FieldType::FieldTypeVec { .. }) => {
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
          .fold(TokenStream::new(), |mut sum, val| {
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
    .fold(TokenStream::new(), |mut sum, val| {
      sum.append_all(val);
      sum
    });

  let match_to_enum: TokenStream = data_enum
    .variants
    .iter()
    .map(|variant| {
      let field_attrs = YaSerdeAttribute::parse(&variant.attrs);
      let renamed_label = match field_attrs.rename {
        Some(value) => Ident::new(&value.to_string(), Span::call_site()),
        None => variant.ident.clone(),
      };
      let label = &variant.ident;
      let label_name = renamed_label.to_string();

      match variant.fields {
        Fields::Unit => Some(quote! {
          #label_name => {
            simple_enum_value = Some(#name::#label);
          }
        }),
        _ => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(TokenStream::new(), |mut tokens, token| {
      tokens.append_all(token);
      tokens
    });

  quote! {
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

        loop {
          match reader.peek()?.to_owned() {
            XmlEvent::StartElement{name, attributes, namespace: _namespace} => {
              debug!("Enum: {}: {}", named_element, name.local_name.as_str());
              if name.local_name == named_element {
                let _next = reader.next_event();

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
              let _root = reader.next_event();
            },
            xml::reader::XmlEvent::Characters(characters_content) => {
              let _root = reader.next_event();
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
