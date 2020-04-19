use crate::attribute::*;
use crate::field_type::*;
use crate::ser::{
  implement_deserializer::implement_deserializer,
  label::build_label_name,
};
use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::DataEnum;
use syn::Fields;
use syn::Ident;

pub fn serialize(
  data_enum: &DataEnum,
  name: &Ident,
  root: &str,
  root_attributes: &YaSerdeAttribute,
) -> TokenStream {
  let inner_enum_inspector = inner_enum_inspector(data_enum, name, root_attributes);

  implement_deserializer(
    name,
    root,
    root_attributes,
    quote!(),
    quote!(match self {
      #inner_enum_inspector
    }),
  )
}

fn inner_enum_inspector(
  data_enum: &DataEnum,
  name: &Ident,
  root_attributes: &YaSerdeAttribute,
  ) -> TokenStream {
  data_enum
    .variants
    .iter()
    .map(|variant| {
      let variant_attrs = YaSerdeAttribute::parse(&variant.attrs);

      let label = &variant.ident;
      let label_name = build_label_name(&label, &variant_attrs, &root_attributes.default_namespace);

      match variant.fields {
        Fields::Unit => Some(quote! {
          &#name::#label => {
            let data_event = XmlEvent::characters(#label_name);
            writer.write(data_event).map_err(|e| e.to_string())?;
          }
        }),
        Fields::Named(ref fields) => {
          let enum_fields: TokenStream = fields
            .named
            .iter()
            .map(|field| {
              let field_attrs = YaSerdeAttribute::parse(&field.attrs);
              if field_attrs.attribute {
                return None;
              }

              let field_label = &field.ident;
              if field_attrs.text {
                return Some(quote!(
                  let data_event = XmlEvent::characters(&self.#field_label);
                  writer.write(data_event).map_err(|e| e.to_string())?;
                ));
              }

              let renamed_field_label = match field_attrs.rename {
                Some(value) => Some(Ident::new(&value.replace("\"", ""), field.span())),
                None => field.ident.clone(),
              };
              let field_label_name = renamed_field_label.unwrap().to_string();

              match get_field_type(field) {
                Some(FieldType::FieldTypeString)
                | Some(FieldType::FieldTypeBool)
                | Some(FieldType::FieldTypeU8)
                | Some(FieldType::FieldTypeI8)
                | Some(FieldType::FieldTypeU16)
                | Some(FieldType::FieldTypeI16)
                | Some(FieldType::FieldTypeU32)
                | Some(FieldType::FieldTypeI32)
                | Some(FieldType::FieldTypeF32)
                | Some(FieldType::FieldTypeU64)
                | Some(FieldType::FieldTypeI64)
                | Some(FieldType::FieldTypeF64) => Some({
                  quote! {
                    match self {
                      &#name::#label{ref #field_label, ..} => {
                        let struct_start_event = XmlEvent::start_element(#field_label_name);
                        writer.write(struct_start_event).map_err(|e| e.to_string())?;

                        let string_value = #field_label.to_string();
                        let data_event = XmlEvent::characters(&string_value);
                        writer.write(data_event).map_err(|e| e.to_string())?;

                        let struct_end_event = XmlEvent::end_element();
                        writer.write(struct_end_event).map_err(|e| e.to_string())?;
                      },
                      _ => {},
                    }
                  }
                }),
                Some(FieldType::FieldTypeStruct { .. }) => Some(quote! {
                  match self {
                    &#name::#label{ref #field_label, ..} => {
                      writer.set_start_event_name(Some(#field_label_name.to_string()));
                      writer.set_skip_start_end(false);
                      #field_label.serialize(writer)?;
                    },
                    _ => {}
                  }
                }),
                Some(FieldType::FieldTypeVec { .. }) => Some(quote! {
                  match self {
                    &#name::#label{ref #field_label, ..} => {
                      for item in #field_label {
                        writer.set_start_event_name(Some(#field_label_name.to_string()));
                        writer.set_skip_start_end(false);
                        item.serialize(writer)?;
                      }
                    },
                    _ => {}
                  }
                }),
                _ => None,
              }
            })
            .filter_map(|x| x)
            .collect();

          Some(quote! {
            &#name::#label{..} => {
              #enum_fields
            }
          })
        }
        Fields::Unnamed(ref fields) => {
          let enum_fields: TokenStream = fields
            .unnamed
            .iter()
            .map(|field| {
              let field_attrs = YaSerdeAttribute::parse(&field.attrs);
              if field_attrs.attribute {
                return None;
              }

              let write_element = |action: &TokenStream| {
                quote! {
                  let struct_start_event = XmlEvent::start_element(#label_name);
                  writer.write(struct_start_event).map_err(|e| e.to_string())?;

                  #action

                  let struct_end_event = XmlEvent::end_element();
                  writer.write(struct_end_event).map_err(|e| e.to_string())?;
                }
              };

              let write_string_chars = quote! {
                let data_event = XmlEvent::characters(item);
                writer.write(data_event).map_err(|e| e.to_string())?;
              };

              let write_simple_type = write_element(&quote! {
                let s = item.to_string();
                let data_event = XmlEvent::characters(&s);
                writer.write(data_event).map_err(|e| e.to_string())?;
              });

              let serialize = quote! {
                writer.set_start_event_name(None);
                writer.set_skip_start_end(true);
                item.serialize(writer)?;
              };

              let write_sub_type = |data_type| {
                write_element(match data_type {
                  FieldType::FieldTypeString => &write_string_chars,
                  _ => &serialize,
                })
              };

              let match_field = |write: &TokenStream| {
                quote! {
                  match self {
                    &#name::#label(ref item) => {
                      #write
                    },
                    _ => {},
                  }
                }
              };

              match get_field_type(field) {
                Some(FieldType::FieldTypeOption { data_type }) => {
                  let write = write_sub_type(*data_type);

                  Some(match_field(&quote! {
                    if let Some(item) = item {
                      #write
                    }
                  }))
                }
                Some(FieldType::FieldTypeVec { data_type }) => {
                  let write = write_sub_type(*data_type);

                  Some(match_field(&quote! {
                    for item in item {
                      #write
                    }
                  }))
                }
                Some(FieldType::FieldTypeStruct { .. }) => {
                  Some(write_element(&match_field(&serialize)))
                }
                Some(FieldType::FieldTypeString) => {
                  Some(match_field(&write_element(&write_string_chars)))
                }
                Some(_simple_type) => Some(match_field(&write_simple_type)),
                _ => None,
              }
            })
            .filter_map(|x| x)
            .collect();

          Some(quote! {
            &#name::#label{..} => {
              #enum_fields
            }
          })
        }
      }
    })
    .filter_map(|x| x)
    .collect()
}
