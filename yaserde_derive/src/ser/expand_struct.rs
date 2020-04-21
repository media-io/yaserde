use crate::common::{Field, YaSerdeAttribute};

use crate::ser::{element::*, implement_deserializer::implement_deserializer};
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
      if !Field::is_attribute(field) {
        return None;
      }

      let label = Field::label(field);
      let label_name = Field::renamed_label(field, root_attributes);

      match Field::from(field) {
        Field::FieldString
        | Field::FieldBool
        | Field::FieldI8
        | Field::FieldU8
        | Field::FieldI16
        | Field::FieldU16
        | Field::FieldI32
        | Field::FieldU32
        | Field::FieldI64
        | Field::FieldU64
        | Field::FieldF32
        | Field::FieldF64 => {
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
        Field::FieldOption { data_type } => match *data_type {
          Field::FieldString => {
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
          Field::FieldBool
          | Field::FieldI8
          | Field::FieldU8
          | Field::FieldI16
          | Field::FieldU16
          | Field::FieldI32
          | Field::FieldU32
          | Field::FieldI64
          | Field::FieldU64
          | Field::FieldF32
          | Field::FieldF64 => {
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
          Field::FieldVec { .. } => {
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
          Field::FieldStruct { .. } => {
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
        Field::FieldStruct { .. } => {
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
      }
    })
    .filter_map(|x| x)
    .collect();

  let struct_inspector: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      if Field::is_attribute(field) {
        return None;
      }

      let label = Field::label(field);
      if Field::is_text_content(field) {
        return Some(quote!(
          let data_event = XmlEvent::characters(&self.#label);
          writer.write(data_event).map_err(|e| e.to_string())?;
        ));
      }

      let label_name = Field::renamed_label(field, root_attributes);

      let conditions = condition_generator(&label, &field_attrs);

      match Field::from(field) {
        Field::FieldString
        | Field::FieldBool
        | Field::FieldI8
        | Field::FieldU8
        | Field::FieldI16
        | Field::FieldU16
        | Field::FieldI32
        | Field::FieldU32
        | Field::FieldI64
        | Field::FieldU64
        | Field::FieldF32
        | Field::FieldF64 => serialize_element(&label, label_name, &conditions),

        Field::FieldOption { data_type } => match *data_type {
          Field::FieldString
          | Field::FieldBool
          | Field::FieldI8
          | Field::FieldU8
          | Field::FieldI16
          | Field::FieldU16
          | Field::FieldI32
          | Field::FieldU32
          | Field::FieldI64
          | Field::FieldU64
          | Field::FieldF32
          | Field::FieldF64 => {
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
          Field::FieldVec { .. } => {
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
          Field::FieldStruct { .. } => Some(if field_attrs.flatten {
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
        Field::FieldStruct { .. } => {
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
        Field::FieldVec { data_type } => match *data_type {
          Field::FieldString => {
            let item_ident = Ident::new("yaserde_item", field.span());
            let inner = enclose_formatted_characters_for_value(&item_ident, label_name);

            Some(quote! {
              for yaserde_item in &self.#label {
                #inner
              }
            })
          }
          Field::FieldBool
          | Field::FieldI8
          | Field::FieldU8
          | Field::FieldI16
          | Field::FieldU16
          | Field::FieldI32
          | Field::FieldU32
          | Field::FieldI64
          | Field::FieldU64
          | Field::FieldF32
          | Field::FieldF64 => {
            let item_ident = Ident::new("yaserde_item", field.span());
            let inner = enclose_formatted_characters_for_value(&item_ident, label_name);

            Some(quote! {
              for yaserde_item in &self.#label {
                #inner
              }
            })
          }
          Field::FieldOption { .. } => Some(quote! {
            for item in &self.#label {
              if let Some(value) = item {
                writer.set_start_event_name(None);
                writer.set_skip_start_end(false);
                value.serialize(writer)?;
              }
            }
          }),
          Field::FieldStruct { .. } => Some(quote! {
            for item in &self.#label {
              writer.set_start_event_name(None);
              writer.set_skip_start_end(false);
              item.serialize(writer)?;
            }
          }),
          Field::FieldVec { .. } => {
            unimplemented!();
          }
        },
      }
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
