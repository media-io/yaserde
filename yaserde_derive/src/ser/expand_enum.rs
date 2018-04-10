
use attribute::*;
use field_type::*;
use quote::Tokens;
use syn::Fields;
use syn::Ident;
use syn::DataEnum;
use proc_macro2::Span;

pub fn serialize(data_enum: &DataEnum, name: &Ident, root: &String) -> Tokens {
  let write_enum_content : Tokens = data_enum.variants.iter().map(|ref variant|
    {
      let field_attrs = YaSerdeAttribute::parse(&variant.attrs);
      let renamed_label =
        match field_attrs.rename {
          Some(value) => Ident::new(&format!("{}", value), Span::call_site()),
          None => variant.ident
        };
      let label = variant.ident;
      let label_name = renamed_label.to_string();

      match variant.fields {
        Fields::Unit => {
          Some(quote!{
            &#name::#label => {
              let data_event = XmlEvent::characters(#label_name);
              let _ret = writer.write(data_event);
            }
          })
        },
        Fields::Named(ref fields) => {
          let enum_fields = fields.named.iter().map(|ref field| {

            let field_attrs = YaSerdeAttribute::parse(&field.attrs);
            if field_attrs.attribute == true {
              return None;
            }

            let field_label = field.ident;
            if field_attrs.text == true {
              return Some(quote!(
                let data_event = XmlEvent::characters(&self.#field_label);
                let _ret = writer.write(data_event);
              ))
            }

            let renamed_field_label =
              match field_attrs.rename {
                Some(value) => Some(Ident::new(&format!("{}", value), Span::call_site())),
                None => field.ident
              };
            let field_label_name = renamed_field_label.unwrap().to_string();


            match get_field_type(field) {
              Some(FieldType::FieldTypeString) =>
                Some(quote!{
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
              Some(FieldType::FieldTypeStruct{..}) =>
                Some(quote!{
                  match self {
                    &#name::#label{ref #field_label, ..} => {
                      match #field_label.derive_serialize(writer) {
                        Ok(()) => {},
                        Err(msg) => {
                          return Err(msg);
                        },
                      };
                    },
                    _ => {}
                  }
                }),
              Some(FieldType::FieldTypeVec{..}) =>
                Some(quote!{
                  match self {
                    &#name::#label{ref #field_label, ..} => {
                      for item in #field_label {
                        match item.derive_serialize(writer) {
                          Ok(()) => {},
                          Err(msg) => {
                            return Err(msg);
                          },
                        };
                      }
                    },
                    _ => {}
                  }
                }),
              _ => None
            }
          })
          .filter(|x| x.is_some())
          .map(|x| x.unwrap())
          .fold(Tokens::new(), |mut tokens, token| {tokens.append_all(token); tokens});

          Some(quote!{
            &#name::#label{..} => {
              let struct_start_event = XmlEvent::start_element(#label_name);
              let _ret = writer.write(struct_start_event);

              #enum_fields

              let struct_end_event = XmlEvent::end_element();
              let _ret = writer.write(struct_end_event);
            }
          })
        },
        Fields::Unnamed(ref _fields) => {
          unimplemented!()
        },
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut tokens, token| {tokens.append_all(token); tokens});

  // println!("{:?}", write_enum_content);

  quote! {
    use xml::writer::XmlEvent;

    impl YaSerialize for #name {
      #[allow(unused_variables)]
      fn derive_serialize<W: Write>(&self, writer: &mut xml::EventWriter<W>) -> Result<(), String> {
        let struct_start_event = XmlEvent::start_element(#root);
        let _ret = writer.write(struct_start_event);
        match self {
          #write_enum_content
        }

        let struct_end_event = XmlEvent::end_element();
        let _ret = writer.write(struct_end_event);
        Ok(())
      }
    }
  }
}
