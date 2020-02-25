use attribute::*;
use field_type::*;
use proc_macro2::TokenStream;
use std::collections::BTreeMap;
use syn::spanned::Spanned;
use syn::DataEnum;
use syn::Fields;
use syn::Ident;

pub fn serialize(
  data_enum: &DataEnum,
  name: &Ident,
  root: &str,
  namespaces: &BTreeMap<String, String>,
  default_namespace: &Option<String>,
) -> TokenStream {
  let write_enum_content: TokenStream = data_enum
    .variants
    .iter()
    .map(|variant| {
      let label = &variant.ident;
      let label_name = {
        let variant_attrs = YaSerdeAttribute::parse(&variant.attrs);
        let prefix = variant_attrs.prefix.map_or(String::new(), |p| p + ":");
        let renamed_label = variant_attrs
          .rename
          .unwrap_or_else(|| variant.ident.to_string());

        prefix + renamed_label.as_str()
      };

      match variant.fields {
        Fields::Unit => Some(quote! {
          &#name::#label => {
            let data_event = XmlEvent::characters(#label_name);
            let _ret = writer.write(data_event);
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
                  let _ret = writer.write(data_event);
                ));
              }

              let renamed_field_label = match field_attrs.rename {
                Some(value) => Some(Ident::new(&value.replace("\"", ""), field.span())),
                None => field.ident.clone(),
              };
              let field_label_name = renamed_field_label.unwrap().to_string();

              match get_field_type(field) {
                Some(FieldType::FieldTypeString) => Some(quote! {
                  match self {
                    &#name::#label{ref #field_label, ..} => {
                      let struct_start_event = XmlEvent::start_element(#field_label_name);
                      let _ret = writer.write(struct_start_event);

                      let data_event = XmlEvent::characters(#field_label);
                      let _ret = writer.write(data_event);

                      let struct_end_event = XmlEvent::end_element();
                      let _ret = writer.write(struct_end_event);
                    },
                    _ => {},
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
                  let _ret = writer.write(struct_start_event);

                  #action

                  let struct_end_event = XmlEvent::end_element();
                  let _ret = writer.write(struct_end_event);
                }
              };

              let write_string_chars = quote! {
                let data_event = XmlEvent::characters(item);
                let _ret = writer.write(data_event);
              };

              let write_simple_type = write_element(&quote! {
                let s = item.to_string();
                let data_event = XmlEvent::characters(&s);
                let _ret = writer.write(data_event);
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
    .collect();

  let add_namespaces: TokenStream = namespaces
    .iter()
    .map(|(prefix, namespace)| {
      if let Some(dn) = default_namespace {
        if dn == prefix {
          return Some(quote!(
            .default_ns(#namespace)
          ));
        }
      }
      Some(quote!(
        .ns(#prefix, #namespace)
      ))
    })
    .filter_map(|x| x)
    .collect();

  quote! {
    use xml::writer::XmlEvent;

    impl YaSerialize for #name {
      #[allow(unused_variables)]
      fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>)
        -> Result<(), String> {
        let skip = writer.skip_start_end();

        if !skip {
          if let Some(label) = writer.get_start_event_name() {
            let struct_start_event = XmlEvent::start_element(label.as_ref());
            let _ret = writer.write(struct_start_event);
          } else {
            let struct_start_event = XmlEvent::start_element(#root)#add_namespaces;
            let _ret = writer.write(struct_start_event);
          }
        }

        match self {
          #write_enum_content
        }

        if !skip {
          let struct_end_event = XmlEvent::end_element();
          let _ret = writer.write(struct_end_event);
        }

        Ok(())
      }
    }
  }
}
