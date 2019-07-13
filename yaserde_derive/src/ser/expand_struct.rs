use attribute::*;
use field_type::*;
use proc_macro2::{Span, TokenStream};
use quote::TokenStreamExt;
use std::collections::BTreeMap;
use std::string::ToString;
use syn::DataStruct;
use syn::Ident;

use ser::element::*;

pub fn serialize(
  data_struct: &DataStruct,
  name: &Ident,
  root: &str,
  namespaces: &BTreeMap<String, String>,
) -> TokenStream {
  let build_attributes: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      if !field_attrs.attribute {
        return None;
      }

      let renamed_label = match field_attrs.rename {
        Some(value) => Ident::new(&value, Span::call_site()),
        None => field.ident.clone().unwrap(),
      };
      let label = &field.ident;
      let label_name = if let Some(prefix) = field_attrs.prefix {
        prefix + ":" + renamed_label.to_string().as_ref()
      } else {
        renamed_label.to_string()
      };

      match get_field_type(field) {
        Some(FieldType::FieldTypeString)
        | Some(FieldType::FieldTypeBool)
        | Some(FieldType::FieldTypeI8)
        | Some(FieldType::FieldTypeU8)
        | Some(FieldType::FieldTypeI16)
        | Some(FieldType::FieldTypeU16)
        | Some(FieldType::FieldTypeI32)
        | Some(FieldType::FieldTypeU32)
        | Some(FieldType::FieldTypeI64)
        | Some(FieldType::FieldTypeU64)
        | Some(FieldType::FieldTypeF32)
        | Some(FieldType::FieldTypeF64) => {
          if let Some(ref d) = field_attrs.default {
            let default_function = Ident::new(&d, Span::call_site());
            Some(quote! {
              let struct_start_event =
                if self.#label != #default_function() {
                  struct_start_event.attr(#label_name, &*{
                    use std::mem;
                    unsafe {
                      let content = format!("{}", self.#label);
                      let ret : &'static str = mem::transmute(&content as &str);
                      mem::forget(content);
                      ret
                    }
                  })
                } else {
                  struct_start_event
                };
            })
          } else {
            Some(quote! {
              let struct_start_event = struct_start_event.attr(#label_name, &*{
                use std::mem;
                unsafe {
                  let content = format!("{}", self.#label);
                  let ret : &'static str = mem::transmute(&content as &str);
                  mem::forget(content);
                  ret
                }
              });
            })
          }
        }
        Some(FieldType::FieldTypeOption { data_type }) => {
          let dt = Box::into_raw(data_type);
          match unsafe { dt.as_ref() } {
            Some(&FieldType::FieldTypeString) => {
              if let Some(ref d) = field_attrs.default {
                let default_function = Ident::new(&d, Span::call_site());
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
            Some(&FieldType::FieldTypeBool)
            | Some(&FieldType::FieldTypeI8)
            | Some(&FieldType::FieldTypeU8)
            | Some(&FieldType::FieldTypeI16)
            | Some(&FieldType::FieldTypeU16)
            | Some(&FieldType::FieldTypeI32)
            | Some(&FieldType::FieldTypeU32)
            | Some(&FieldType::FieldTypeI64)
            | Some(&FieldType::FieldTypeU64)
            | Some(&FieldType::FieldTypeF32)
            | Some(&FieldType::FieldTypeF64) => {
              if let Some(ref d) = field_attrs.default {
                let default_function = Ident::new(&d, Span::call_site());
                Some(quote! {
                  let struct_start_event =
                    if self.#label != #default_function() {
                      if let Some(ref value) = self.#label {
                        struct_start_event.attr(#label_name, &*{
                          use std::mem;
                          unsafe {
                            let content = format!("{}", value);
                            let ret : &'static str = mem::transmute(&content as &str);
                            mem::forget(content);
                            ret
                          }
                        })
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
                      struct_start_event.attr(#label_name, &*{
                        use std::mem;
                        unsafe {
                          let content = format!("{}", value);
                          let ret : &'static str = mem::transmute(&content as &str);
                          mem::forget(content);
                          ret
                        }
                      })
                    } else {
                      struct_start_event
                    };
                })
              }
            }
            Some(&FieldType::FieldTypeVec { .. }) => {
              let item_ident = Ident::new("yas_item", Span::call_site());
              let inner = enclose_formatted_characters(&item_ident, label_name);

              if let Some(ref d) = field_attrs.default {
                let default_function = Ident::new(&d, Span::call_site());

                Some(quote! {
                  if self.#label != #default_function() {
                    if let Some(ref yas_list) = self.#label {
                      for yas_item in yas_list.iter() {
                        #inner
                      }
                    }
                  }
                })
              } else {
                Some(quote! {
                  for yas_item in &self.#label {
                    #inner
                  }
                })
              }
            }
            _ => unimplemented!(),
          }
        }
        Some(FieldType::FieldTypeStruct { .. }) => {
          if let Some(ref d) = field_attrs.default {
            let default_function = Ident::new(&d, Span::call_site());
            Some(quote! {
              let struct_start_event =
                if self.#label != #default_function() {
                  struct_start_event.attr(#label_name, &*{
                    use std::mem;
                    match yaserde::ser::to_string_content(&self.#label) {
                      Ok(value) => {
                        unsafe {
                          let ret : &'static str = mem::transmute(&value as &str);
                          mem::forget(value);
                          ret
                        }
                      },
                      Err(msg) => return Err("Unable to serialize content".to_owned()),
                    }
                  })
                } else {
                  struct_start_event
                };
            })
          } else {
            Some(quote! {
              let struct_start_event = struct_start_event.attr(#label_name, &*{
                use std::mem;
                match yaserde::ser::to_string_content(&self.#label) {
                  Ok(value) => {
                    unsafe {
                      let ret : &'static str = mem::transmute(&value as &str);
                      mem::forget(value);
                      ret
                    }
                  },
                  Err(msg) => return Err("Unable to serialize content".to_owned()),
                }
              });
            })
          }
        }
        _ => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(TokenStream::new(), |mut tokens, token| {
      tokens.append_all(token);
      tokens
    });

  let add_namespaces: TokenStream = namespaces
    .iter()
    .map(|(prefix, namespace)| {
      Some(quote!(
        .ns(#prefix, #namespace)
      ))
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(TokenStream::new(), |mut tokens, token| {
      tokens.append_all(token);
      tokens
    });

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
          let _ret = writer.write(data_event);
        ));
      }

      let renamed_label = match field_attrs.rename {
        Some(value) => Ident::new(&value, Span::call_site()),
        None => field.ident.clone().unwrap(),
      };

      let label_name = if let Some(prefix) = field_attrs.prefix {
        format!("{}:{}", prefix, renamed_label)
      } else {
        renamed_label.to_string()
      };

      match get_field_type(field) {
        Some(FieldType::FieldTypeString)
        | Some(FieldType::FieldTypeBool)
        | Some(FieldType::FieldTypeI8)
        | Some(FieldType::FieldTypeU8)
        | Some(FieldType::FieldTypeI16)
        | Some(FieldType::FieldTypeU16)
        | Some(FieldType::FieldTypeI32)
        | Some(FieldType::FieldTypeU32)
        | Some(FieldType::FieldTypeI64)
        | Some(FieldType::FieldTypeU64)
        | Some(FieldType::FieldTypeF32)
        | Some(FieldType::FieldTypeF64) => {
          serialize_element(label, label_name, &field_attrs.default)
        }
        Some(FieldType::FieldTypeOption { data_type }) => {
          let dt = Box::into_raw(data_type);
          match unsafe { dt.as_ref() } {
            Some(&FieldType::FieldTypeString)
            | Some(&FieldType::FieldTypeBool)
            | Some(&FieldType::FieldTypeI8)
            | Some(&FieldType::FieldTypeU8)
            | Some(&FieldType::FieldTypeI16)
            | Some(&FieldType::FieldTypeU16)
            | Some(&FieldType::FieldTypeI32)
            | Some(&FieldType::FieldTypeU32)
            | Some(&FieldType::FieldTypeI64)
            | Some(&FieldType::FieldTypeU64)
            | Some(&FieldType::FieldTypeF32)
            | Some(&FieldType::FieldTypeF64) => {
              let item_ident = Ident::new("yas_item", Span::call_site());
              let inner = enclose_formatted_characters_for_value(&item_ident, label_name);

              if let Some(ref d) = field_attrs.default {
                let default_function = Ident::new(&d, Span::call_site());

                Some(quote! {
                  if self.#label != #default_function() {
                    if let Some(ref yas_item) = self.#label {
                      #inner
                    }
                  }
                })
              } else {
                Some(quote! {
                  if let Some(ref yas_item) = self.#label {
                    #inner
                  }
                })
              }
            }
            Some(&FieldType::FieldTypeVec { .. }) => {
              let item_ident = Ident::new("yas_item", Span::call_site());
              let inner = enclose_formatted_characters_for_value(&item_ident, label_name);

              if let Some(ref d) = field_attrs.default {
                let default_function = Ident::new(&d, Span::call_site());

                Some(quote! {
                  if self.#label != #default_function() {
                    if let Some(ref yas_items) = &self.#label {
                      for yas_item in yas_items.iter() {
                        #inner
                      }
                    }
                  }
                })
              } else {
                Some(quote! {
                  if let Some(ref yas_items) = &self.#label {
                    for yas_item in yas_items.iter() {
                      #inner
                    }
                  }
                })
              }
            }
            Some(&FieldType::FieldTypeStruct { .. }) => Some(quote! {
              if let Some(ref item) = &self.#label {
                writer.set_start_event_name(Some(#label_name.to_string()));
                match item.serialize(writer) {
                  Ok(()) => {},
                  Err(msg) => {
                    return Err(msg);
                  },
                };
                writer.set_start_event_name(None);

                writer.set_skip_start_end(true);
                match item.serialize(writer) {
                  Ok(()) => {},
                  Err(msg) => {
                    return Err(msg);
                  },
                };

                let end_event = XmlEvent::end_element();
                let _ret = writer.write(end_event);
              }
            }),
            _ => unimplemented!(),
          }
        }
        Some(FieldType::FieldTypeStruct { .. }) => Some(quote! {
          writer.set_start_event_name(Some(#label_name.to_string()));
          match self.#label.serialize(writer) {
            Ok(()) => {},
            Err(msg) => {
              return Err(msg);
            },
          };
          writer.set_start_event_name(None);

          writer.set_skip_start_end(true);
          match self.#label.serialize(writer) {
            Ok(()) => {},
            Err(msg) => {
              return Err(msg);
            },
          };

          let end_event = XmlEvent::end_element();
          let _ret = writer.write(end_event);
        }),
        Some(FieldType::FieldTypeVec { data_type }) => {
          let dt = Box::into_raw(data_type);
          match unsafe { dt.as_ref() } {
            Some(&FieldType::FieldTypeString) => {
              let item_ident = Ident::new("yas_item", Span::call_site());
              let inner = enclose_formatted_characters_for_value(&item_ident, label_name);

              Some(quote! {
                for yas_item in &self.#label {
                  #inner
                }
              })
            }
            Some(&FieldType::FieldTypeBool)
            | Some(&FieldType::FieldTypeI8)
            | Some(&FieldType::FieldTypeU8)
            | Some(&FieldType::FieldTypeI16)
            | Some(&FieldType::FieldTypeU16)
            | Some(&FieldType::FieldTypeI32)
            | Some(&FieldType::FieldTypeU32)
            | Some(&FieldType::FieldTypeI64)
            | Some(&FieldType::FieldTypeU64)
            | Some(&FieldType::FieldTypeF32)
            | Some(&FieldType::FieldTypeF64) => {
              let item_ident = Ident::new("yas_item", Span::call_site());
              let inner = enclose_formatted_characters_for_value(&item_ident, label_name);

              Some(quote! {
                for yas_item in &self.#label {
                  #inner
                }
              })
            }
            Some(&FieldType::FieldTypeOption { .. }) => Some(quote! {
              for item in &self.#label {
                if let Some(value) = item {
                  writer.set_skip_start_end(false);
                  match value.serialize(writer) {
                    Ok(()) => {},
                    Err(msg) => {
                      return Err(msg);
                    },
                  };
                }
              }
            }),
            Some(&FieldType::FieldTypeStruct { .. }) => Some(quote! {
              for item in &self.#label {
                writer.set_skip_start_end(false);
                match item.serialize(writer) {
                  Ok(()) => {},
                  Err(msg) => {
                    return Err(msg);
                  },
                };
              }
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
    .fold(TokenStream::new(), |mut tokens, token| {
      tokens.append_all(token);
      tokens
    });

  quote! {
    use xml::writer::XmlEvent;

    impl YaSerialize for #name {
      #[allow(unused_variables)]
      fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>)
        -> Result<(), String> {
        if let Some(label) = writer.get_start_event_name() {
          let struct_start_event = XmlEvent::start_element(label.as_ref());
          #build_attributes
          let _ret = writer.write(struct_start_event);
          return Ok(())
        }
        error!("Struct: start to expand {:?}", #root);
        let skip = writer.skip_start_end();
        if !skip {
          let struct_start_event = XmlEvent::start_element(#root)#add_namespaces;
          #build_attributes
          let _ret = writer.write(struct_start_event);
        }

        #struct_inspector

        if !skip {
          let struct_end_event = XmlEvent::end_element();
          let _ret = writer.write(struct_end_event);
        }
        Ok(())
      }
    }
  }
}
