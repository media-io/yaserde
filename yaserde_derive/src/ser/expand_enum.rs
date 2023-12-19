use crate::common::{Field, YaSerdeAttribute, YaSerdeField};
use crate::ser::{implement_serializer::implement_serializer, label::build_label_name};
use proc_macro2::TokenStream;
use quote::quote;
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

  let get_id = |field: &YaSerdeField| {
    field
      .label()
      .unwrap_or(field.get_type().get_simple_type_visitor())
  };

  let variant_matches: TokenStream = data_enum
    .variants
    .iter()
    .map(|variant| -> TokenStream {
      let _attrs = crate::common::YaSerdeAttribute::parse(&variant.attrs);

      let all_fields = variant
        .fields
        .iter()
        .map(|field| YaSerdeField::new(field.clone()));

      let attribute_fields: Vec<_> = all_fields
        .clone()
        .into_iter()
        .filter(|field| {
          field.is_attribute()
            || (field.is_flatten() && matches!(field.get_type(), Field::FieldStruct { .. }))
        })
        .collect();

      attribute_fields
        .iter()
        .map(|field| {
          let label = variant.ident.clone();
          let var = get_id(field);
          let name = name.clone();

          let destructure = if field.get_value_label().is_some() {
            quote! {{#var, ..}}
          } else {
            quote! {(#var, ..)}
          };

          if field.is_attribute() {
            quote! { #name::#label { .. } => { }, }
          } else {
            match field.get_type() {
              Field::FieldStruct { .. } => {
                if root_attributes.flatten {
                  quote! {
                    match self {
                      #name::#label #destructure => {
                        let (attributes, namespace) = #var.serialize_attributes(
                          child_attributes,
                          child_attributes_namespace,
                        )?;
                        child_attributes_namespace.extend(&namespace);
                        child_attributes.extend(attributes);
                      },
                      _ => {}
                    }
                  }
                } else {
                  quote! {}
                }
              }
              _ => quote! { #name::#label { .. } => { },},
            }
          }
        })
        .collect()
    })
    .collect();

  implement_serializer(
    name,
    root,
    root_attributes,
    quote!(#variant_matches),
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
      let label_name = build_label_name(label, &variant_attrs, &root_attributes.default_namespace);

      match variant.fields {
        Fields::Unit => quote! {
          &#name::#label => {
            let data_event = ::yaserde::__xml::writer::XmlEvent::characters(#label_name);
            writer.write(data_event).map_err(|e| e.to_string())?;
          }
        },
        Fields::Named(ref fields) => {
          let enum_fields: TokenStream = fields
            .named
            .iter()
            .map(|field| YaSerdeField::new(field.clone()))
            .filter(|field| !field.is_attribute())
            .filter_map(|field| {
              let field_label = field.label();

              if field.is_text_content() {
                return Some(quote!(
                  let data_event = ::yaserde::__xml::writer::XmlEvent::characters(&self.#field_label);
                  writer.write(data_event).map_err(|e| e.to_string())?;
                ));
              }

              let field_label_name = field.renamed_label(root_attributes);

              match field.get_type() {
                Field::FieldString
                | Field::FieldBool
                | Field::FieldU8
                | Field::FieldI8
                | Field::FieldU16
                | Field::FieldI16
                | Field::FieldU32
                | Field::FieldI32
                | Field::FieldF32
                | Field::FieldU64
                | Field::FieldI64
                | Field::FieldF64 => Some({
                  quote! {
                    match self {
                      &#name::#label { ref #field_label, .. } => {
                        let struct_start_event =
                          ::yaserde::__xml::writer::XmlEvent::start_element(#field_label_name);
                        writer.write(struct_start_event).map_err(|e| e.to_string())?;

                        let string_value = #field_label.to_string();
                        let data_event = ::yaserde::__xml::writer::XmlEvent::characters(&string_value);
                        writer.write(data_event).map_err(|e| e.to_string())?;

                        let struct_end_event = ::yaserde::__xml::writer::XmlEvent::end_element();
                        writer.write(struct_end_event).map_err(|e| e.to_string())?;
                      },
                      _ => {},
                    }
                  }
                }),
                Field::FieldStruct { .. } => Some(quote! {
                  match self {
                    &#name::#label{ref #field_label, ..} => {
                      writer.set_start_event_name(
                        ::std::option::Option::Some(#field_label_name.to_string()),
                      );
                      writer.set_skip_start_end(false);
                      ::yaserde::YaSerialize::serialize(#field_label, writer)?;
                    },
                    _ => {}
                  }
                }),
                Field::FieldVec { .. } => Some(quote! {
                  match self {
                    &#name::#label { ref #field_label, .. } => {
                      for item in #field_label {
                        writer.set_start_event_name(
                          ::std::option::Option::Some(#field_label_name.to_string()),
                        );
                        writer.set_skip_start_end(false);
                        ::yaserde::YaSerialize::serialize(item, writer)?;
                      }
                    },
                    _ => {}
                  }
                }),
                Field::FieldOption { .. } => None,
              }
            })
            .collect();

          quote! {
            &#name::#label{..} => {
              #enum_fields
            }
          }
        }
        Fields::Unnamed(ref fields) => {
          let enum_fields: TokenStream = fields
            .unnamed
            .iter()
            .map(|field| YaSerdeField::new(field.clone()))
            .filter(|field| !field.is_attribute())
            .map(|field| {
              let write_element = |action: &TokenStream| {
                quote! {
                  let struct_start_event = ::yaserde::__xml::writer::XmlEvent::start_element(#label_name);
                  writer.write(struct_start_event).map_err(|e| e.to_string())?;

                  #action

                  let struct_end_event = ::yaserde::__xml::writer::XmlEvent::end_element();
                  writer.write(struct_end_event).map_err(|e| e.to_string())?;
                }
              };

              let write_string_chars = quote! {
                let data_event = ::yaserde::__xml::writer::XmlEvent::characters(item);
                writer.write(data_event).map_err(|e| e.to_string())?;
              };

              let write_simple_type = write_element(&quote! {
                let s = item.to_string();
                let data_event = ::yaserde::__xml::writer::XmlEvent::characters(&s);
                writer.write(data_event).map_err(|e| e.to_string())?;
              });

              let serialize = quote! {
                writer.set_start_event_name(::std::option::Option::None);
                writer.set_skip_start_end(true);
                ::yaserde::YaSerialize::serialize(item, writer)?;
              };

              let write_sub_type = |data_type| {
                write_element(match data_type {
                  Field::FieldString => &write_string_chars,
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

              match field.get_type() {
                Field::FieldOption { data_type } => {
                  let write = write_sub_type(*data_type);

                  match_field(&quote! {
                    if let ::std::option::Option::Some(item) = item {
                      #write
                    }
                  })
                }
                Field::FieldVec { data_type } => {
                  let write = write_sub_type(*data_type);

                  match_field(&quote! {
                    for item in item {
                      #write
                    }
                  })
                }
                Field::FieldStruct { .. } => {
                  if variant_attrs.flatten || field.is_flatten() {
                     match_field(&quote!{ ::yaserde::YaSerialize::serialize(item, writer)?})
                   } else {
                     write_element(&match_field(&serialize))
                   }
                }
                Field::FieldString => match_field(&write_element(&write_string_chars)),
                _simple_type => match_field(&write_simple_type),
              }
            })
            .collect();

          quote! {
            &#name::#label{..} => {
              #enum_fields
            }
          }
        }
      }
    })
    .collect()
}
