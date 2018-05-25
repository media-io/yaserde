use attribute::*;
use field_type::*;
use quote::TokenStreamExt;
use std::collections::BTreeMap;
use syn::Fields;
use syn::Ident;
use syn::DataEnum;
use proc_macro2::{Span, TokenStream};

pub fn serialize(
  data_enum: &DataEnum,
  name: &Ident,
  root: &str,
  namespaces: &BTreeMap<String, String>,
) -> TokenStream {
  let write_enum_content: TokenStream = data_enum
    .variants
    .iter()
    .map(|variant| {
      let variant_attrs = YaSerdeAttribute::parse(&variant.attrs);
      let renamed_label = match variant_attrs.rename {
        Some(value) => Ident::new(&format!("{}", value), Span::call_site()),
        None => variant.ident.clone(),
      };
      let label = &variant.ident;
      let label_name = if let Some(prefix) = variant_attrs.prefix {
        prefix + ":" + renamed_label.to_string().as_ref()
      } else {
        renamed_label.to_string()
      };

      match variant.fields {
        Fields::Unit => Some(quote!{
          &#name::#label => {
            let data_event = XmlEvent::characters(#label_name);
            let _ret = writer.write(data_event);
          }
        }),
        Fields::Named(ref fields) => {
          let enum_fields = fields
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
                Some(value) => Some(Ident::new(&format!("{}", value), Span::call_site())),
                None => field.ident.clone(),
              };
              let field_label_name = renamed_field_label.unwrap().to_string();

              match get_field_type(field) {
                Some(FieldType::FieldTypeString) => Some(quote!{
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
                Some(FieldType::FieldTypeStruct { .. }) => Some(quote!{
                  let struct_start_event = XmlEvent::start_element(#field_label_name);
                  let _ret = writer.write(struct_start_event);

                  match self {
                    &#name::#label{ref #field_label, ..} => {
                      writer.set_skip_start_end(true);
                      if let Err(msg) = #field_label.serialize(writer) {
                        return Err(msg);
                      };
                    },
                    _ => {}
                  }

                  let struct_end_event = XmlEvent::end_element();
                  let _ret = writer.write(struct_end_event);
                }),
                Some(FieldType::FieldTypeVec { .. }) => Some(quote!{
                  match self {
                    &#name::#label{ref #field_label, ..} => {
                      for item in #field_label {
                        let struct_start_event = XmlEvent::start_element(#field_label_name);
                        let _ret = writer.write(struct_start_event);

                        writer.set_skip_start_end(true);
                        if let Err(msg) = item.serialize(writer) {
                          return Err(msg);
                        };
                        let struct_end_event = XmlEvent::end_element();
                        let _ret = writer.write(struct_end_event);
                      }
                    },
                    _ => {}
                  }
                }),
                _ => None,
              }
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .fold(TokenStream::empty(), |mut tokens, token| {
              tokens.append_all(token);
              tokens
            });

          Some(quote!{
            &#name::#label{..} => {
              let struct_start_event = XmlEvent::start_element(#label_name);
              let _ret = writer.write(struct_start_event);

              #enum_fields

              let struct_end_event = XmlEvent::end_element();
              let _ret = writer.write(struct_end_event);
            }
          })
        }
        Fields::Unnamed(ref _fields) => unimplemented!(),
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(TokenStream::empty(), |mut tokens, token| {
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
    .fold(TokenStream::empty(), |mut tokens, token| {
      tokens.append_all(token);
      tokens
    });

  quote! {
    use xml::writer::XmlEvent;

    impl YaSerialize for #name {
      #[allow(unused_variables)]
      fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>)
        -> Result<(), String> {
        error!("Enum: start to expand {:?}", #root);

        if !writer.skip_start_end() {
          let struct_start_event = XmlEvent::start_element(#root)#add_namespaces;
          let _ret = writer.write(struct_start_event);
        }

        match self {
          #write_enum_content
        }

        if !writer.skip_start_end() {
          let struct_end_event = XmlEvent::end_element();
          let _ret = writer.write(struct_end_event);
        }
        writer.set_skip_start_end(false);
        Ok(())
      }
    }
  }
}
