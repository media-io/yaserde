use crate::attribute::*;
use crate::field_type::*;
use crate::ser::{
  element::*,
  implement_deserializer::implement_deserializer,
  label::build_label_name,
};
use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::DataStruct;
use syn::Ident;

pub fn serialize(
  data_struct: &DataStruct,
  name: &Ident,
  root: &str,
  root_attributes: &YaSerdeAttribute,
) -> TokenStream {
  let build_attributes: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      if !field_attrs.attribute {
        return None;
      }

      let label = &field.ident;

      let label_name = build_label_name(&label.as_ref().unwrap(), &field_attrs, &root_attributes.default_namespace);

      get_field_type(field).and_then(|f| match f {
        FieldType::FieldTypeString
        | FieldType::FieldTypeBool
        | FieldType::FieldTypeI8
        | FieldType::FieldTypeU8
        | FieldType::FieldTypeI16
        | FieldType::FieldTypeU16
        | FieldType::FieldTypeI32
        | FieldType::FieldTypeU32
        | FieldType::FieldTypeI64
        | FieldType::FieldTypeU64
        | FieldType::FieldTypeF32
        | FieldType::FieldTypeF64 => {
          if let Some(ref d) = field_attrs.default {
            let default_function = Ident::new(&d, field.span());
            Some(quote! {
              let content = self.#label.to_string();
              let struct_start_event =
                if self.#label != #default_function() {
                  struct_start_event.attr(#label_name, &content)
                } else {
                  struct_start_event
                };
            })
          } else {
            Some(quote! {
              let content = self.#label.to_string();
              let struct_start_event = struct_start_event.attr(#label_name, &content);
            })
          }
        }
        FieldType::FieldTypeOption { data_type } => match *data_type {
          FieldType::FieldTypeString => {
            if let Some(ref d) = field_attrs.default {
              let default_function = Ident::new(&d, field.span());
              Some(quote! {
                let struct_start_event =
                  if self.#label != #default_function() {
                    if let Some(ref value) = self.#label {
                      struct_start_event.attr(#label_name, &value)
                    } else {
                      struct_start_event
                    }
                  } else {
                    struct_start_event
                  };
              })
            } else {
              Some(quote! {
                let struct_start_event =
                  if let Some(ref value) = self.#label {
                    struct_start_event.attr(#label_name, &value)
                  } else {
                    struct_start_event
                  };
              })
            }
          }
          FieldType::FieldTypeBool
          | FieldType::FieldTypeI8
          | FieldType::FieldTypeU8
          | FieldType::FieldTypeI16
          | FieldType::FieldTypeU16
          | FieldType::FieldTypeI32
          | FieldType::FieldTypeU32
          | FieldType::FieldTypeI64
          | FieldType::FieldTypeU64
          | FieldType::FieldTypeF32
          | FieldType::FieldTypeF64 => {
            if let Some(ref d) = field_attrs.default {
              let default_function = Ident::new(&d, field.span());
              Some(quote! {
                let content = self.#label.map_or_else(|| String::new(), |v| v.to_string());
                let struct_start_event =
                  if self.#label != #default_function() {
                    if let Some(ref value) = self.#label {
                      struct_start_event.attr(#label_name, &content)
                    } else {
                      struct_start_event
                    }
                  } else {
                    struct_start_event
                  };
              })
            } else {
              Some(quote! {
                let content = self.#label.map_or_else(|| String::new(), |v| v.to_string());
                let struct_start_event =
                  if let Some(ref value) = self.#label {
                    struct_start_event.attr(#label_name, &content)
                  } else {
                    struct_start_event
                  };
              })
            }
          }
          FieldType::FieldTypeVec { .. } => {
            let item_ident = Ident::new("yaserde_item", field.span());
            let inner = enclose_formatted_characters(&item_ident, label_name);

            if let Some(ref d) = field_attrs.default {
              let default_function = Ident::new(&d, field.span());

              Some(quote! {
                if self.#label != #default_function() {
                  if let Some(ref yaserde_list) = self.#label {
                    for yaserde_item in yaserde_list.iter() {
                      #inner
                    }
                  }
                }
              })
            } else {
              Some(quote! {
                for yaserde_item in &self.#label {
                  #inner
                }
              })
            }
          }
          FieldType::FieldTypeStruct { .. } => {
            if let Some(ref d) = field_attrs.default {
              let default_function = Ident::new(&d, field.span());
              Some(quote! {
                let content = self.#label
                  .as_ref()
                  .map_or_else(|| Ok(String::new()), |v| yaserde::ser::to_string_content(v))?;
                let struct_start_event = if let Some(ref value) = self.#label {
                  if *value != #default_function() {
                    struct_start_event.attr(#label_name, &content)
                  } else {
                    struct_start_event
                  }
                } else {
                  struct_start_event
                };
              })
            } else {
              Some(quote! {
                let content = self.#label
                  .as_ref()
                  .map_or_else(|| Ok(String::new()), |v| yaserde::ser::to_string_content(v))?;
                let struct_start_event = if let Some(ref value) = self.#label {
                  struct_start_event.attr(#label_name, &content)
                } else {
                  struct_start_event
                };
              })
            }
          }
          _ => unimplemented!(),
        },
        FieldType::FieldTypeStruct { .. } => {
          if let Some(ref d) = field_attrs.default {
            let default_function = Ident::new(&d, field.span());
            Some(quote! {
              let content = yaserde::ser::to_string_content(&self.#label)?;
              let struct_start_event =
                if self.#label != #default_function() {
                  struct_start_event.attr(#label_name, &content)
                } else {
                  struct_start_event
                };
            })
          } else {
            Some(quote! {
              let content = yaserde::ser::to_string_content(&self.#label)?;
              let struct_start_event = struct_start_event.attr(#label_name, &content);
            })
          }
        }
        _ => None,
      })
    })
    .filter_map(|x| x)
    .collect();

  let struct_inspector: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      if field_attrs.attribute {
        return None;
      }

      let label = &field.ident;
      if field_attrs.text {
        return Some(quote!(
          let data_event = XmlEvent::characters(&self.#label);
          writer.write(data_event).map_err(|e| e.to_string())?;
        ));
      }

      let label_name = build_label_name(&label.as_ref().unwrap(), &field_attrs, &root_attributes.default_namespace);
      let conditions = condition_generator(label, &field_attrs);

      get_field_type(field).and_then(|f| match f {
        FieldType::FieldTypeString
        | FieldType::FieldTypeBool
        | FieldType::FieldTypeI8
        | FieldType::FieldTypeU8
        | FieldType::FieldTypeI16
        | FieldType::FieldTypeU16
        | FieldType::FieldTypeI32
        | FieldType::FieldTypeU32
        | FieldType::FieldTypeI64
        | FieldType::FieldTypeU64
        | FieldType::FieldTypeF32
        | FieldType::FieldTypeF64 => serialize_element(label, label_name, &conditions),
        FieldType::FieldTypeOption { data_type } => match *data_type {
          FieldType::FieldTypeString
          | FieldType::FieldTypeBool
          | FieldType::FieldTypeI8
          | FieldType::FieldTypeU8
          | FieldType::FieldTypeI16
          | FieldType::FieldTypeU16
          | FieldType::FieldTypeI32
          | FieldType::FieldTypeU32
          | FieldType::FieldTypeI64
          | FieldType::FieldTypeU64
          | FieldType::FieldTypeF32
          | FieldType::FieldTypeF64 => {
            let item_ident = Ident::new("yaserde_item", field.span());
            let inner = enclose_formatted_characters_for_value(&item_ident, label_name);

            Some(quote! {
              #conditions {
                if let Some(ref yaserde_item) = self.#label {
                  #inner
                }
              }
            })
          }
          FieldType::FieldTypeVec { .. } => {
            let item_ident = Ident::new("yaserde_item", field.span());
            let inner = enclose_formatted_characters_for_value(&item_ident, label_name);

            Some(quote! {
              #conditions {
                if let Some(ref yaserde_items) = &self.#label {
                  for yaserde_item in yaserde_items.iter() {
                    #inner
                  }
                }
              }
            })
          }
          FieldType::FieldTypeStruct { .. } => Some(if field_attrs.flatten {
            quote! {
              if let Some(ref item) = &self.#label {
                writer.set_start_event_name(None);
                writer.set_skip_start_end(true);
                item.serialize(writer)?;
              }
            }
          } else {
            quote! {
              if let Some(ref item) = &self.#label {
                writer.set_start_event_name(Some(#label_name.to_string()));
                writer.set_skip_start_end(false);
                item.serialize(writer)?;
              }
            }
          }),
          _ => unimplemented!(),
        },
        FieldType::FieldTypeStruct { .. } => {
          let (start_event, skip_start) = if field_attrs.flatten {
            (quote!(None), true)
          } else {
            (quote!(Some(#label_name.to_string())), false)
          };

          Some(quote! {
            writer.set_start_event_name(#start_event);
            writer.set_skip_start_end(#skip_start);
            self.#label.serialize(writer)?;
          })
        }
        FieldType::FieldTypeVec { data_type } => match *data_type {
          FieldType::FieldTypeString => {
            let item_ident = Ident::new("yaserde_item", field.span());
            let inner = enclose_formatted_characters_for_value(&item_ident, label_name);

            Some(quote! {
              for yaserde_item in &self.#label {
                #inner
              }
            })
          }
          FieldType::FieldTypeBool
          | FieldType::FieldTypeI8
          | FieldType::FieldTypeU8
          | FieldType::FieldTypeI16
          | FieldType::FieldTypeU16
          | FieldType::FieldTypeI32
          | FieldType::FieldTypeU32
          | FieldType::FieldTypeI64
          | FieldType::FieldTypeU64
          | FieldType::FieldTypeF32
          | FieldType::FieldTypeF64 => {
            let item_ident = Ident::new("yaserde_item", field.span());
            let inner = enclose_formatted_characters_for_value(&item_ident, label_name);

            Some(quote! {
              for yaserde_item in &self.#label {
                #inner
              }
            })
          }
          FieldType::FieldTypeOption { .. } => Some(quote! {
            for item in &self.#label {
              if let Some(value) = item {
                writer.set_start_event_name(None);
                writer.set_skip_start_end(false);
                value.serialize(writer)?;
              }
            }
          }),
          FieldType::FieldTypeStruct { .. } => Some(quote! {
            for item in &self.#label {
              writer.set_start_event_name(None);
              writer.set_skip_start_end(false);
              item.serialize(writer)?;
            }
          }),
          FieldType::FieldTypeVec { .. } => {
            unimplemented!();
          }
        },
      })
    })
    .filter_map(|x| x)
    .collect();

  implement_deserializer(
    name,
    root,
    root_attributes,
    build_attributes,
    struct_inspector,
  )
}
