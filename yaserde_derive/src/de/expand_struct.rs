
use de::attribute::*;
use de::field_type::*;
use quote::Tokens;
use syn::Ident;
use syn::DataStruct;
use syn::punctuated::Pair;
use syn::Type::Path;
use proc_macro2::Span;

pub fn parse(data_struct: &DataStruct, name: &Ident, root: &String) -> Tokens {
  let variables : Tokens = data_struct.fields.iter().map(|ref field|
    {
      let label = field.ident;

      match field.ty {
        Path(ref path) => {
          match path.path.segments.first() {
            Some(Pair::End(t)) => {
              let pair = path.path.segments.first().unwrap();

              match t.ident.to_string().as_str() {
                "String" => {
                  Some(quote!{
                    let mut #label : #pair = "".to_string();
                  })
                },
                "Vec" => {
                  Some(quote!{
                    let mut #label : #pair = vec![];
                  })
                },
                _ => {
                  Some(quote!{
                    let mut #label : #pair = #pair::default();
                  })
                },
              }
            },
            _ => {
              None
            },
          }
        },
        _ => {None},
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {sum.append_all(val); sum});

  let attributes_loading: Tokens = data_struct.fields.iter().map(|ref field|
    match get_field_type(field) {
      Some(FieldType::FieldTypeString) => {
        let label = field.ident;
        let field_attrs = YaSerdeAttribute::parse(&field.attrs);

        match (field_attrs.attribute, field_attrs.rename) {
          (true, Some(value)) => {
            let label_name = Ident::new(&format!("{}", value), Span::call_site()).to_string();
            Some(quote!{
              match current_attributes {
                Some(attributes) =>
                  for attr in attributes {
                    if attr.name.local_name == #label_name {
                      #label = attr.value.to_owned();
                    }
                  },
                None => {},
              }
            })
          },
          (true, None) => {
            let label_name = field.ident.unwrap().to_string();
            Some(quote!{
              match current_attributes {
                Some(attributes) =>
                  for attr in attributes {
                    if attr.name.local_name == #label_name {
                      #label = attr.value.to_owned();
                    }
                  },
                None => {},
              }
            })
          }
          _ => None
        }
      }
      _ => {
        None
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {sum.append_all(val); sum});

  let assign_text_field: Tokens = data_struct.fields.iter().map(|ref field|
    match get_field_type(field) {
      Some(FieldType::FieldTypeString) => {
        let label = field.ident;
        let field_attrs = YaSerdeAttribute::parse(&field.attrs);

        match field_attrs.text {
          true => {
            Some(quote!{
              #label = characters_content.to_owned();
            })
          },
          false => None
        }
      }
      _ => {
        None
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {sum.append_all(val); sum});

  let fields : Tokens = data_struct.fields.iter().map(|ref field|
    {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      let label = field.ident;
      let renamed_label =
        match field_attrs.rename {
          Some(value) => Some(Ident::new(&format!("{}", value), Span::call_site())),
          None => field.ident
        };

      let label_name = renamed_label.unwrap().to_string();
      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => {
          Some(quote!{
            #label_name => {
              match read.next() {
                Ok(xml::reader::XmlEvent::Characters(characters_content)) => {
                  #label = characters_content.trim().to_string();
                },
                _ => {},
              }
            },
          })
        },
        Some(FieldType::FieldTypeStruct{name}) => {
          let struct_ident = Ident::new(&format!("{}", name), Span::def_site());

          Some(quote!{
            #label_name => {
              match #struct_ident::derive_deserialize(read, Some(&attributes)) {
                Ok(parsed_structure) => {
                  prev_level -= 1;
                  #label = parsed_structure;
                },
                Err(msg) => {
                  println!("ERROR {:?}", msg);
                },
              }
            },
          })
        },
        Some(FieldType::FieldTypeVec) => {
          match get_vec_type(field) {
            Some(identifier) => {
              match identifier.to_string().as_str() {
                "String" => {
                  Some(quote!{
                    #label_name => {
                      match read.next() {
                        Ok(xml::reader::XmlEvent::Characters(characters_content)) => {
                          #label.push(characters_content.trim().to_string());
                        },
                        _ => {},
                      }
                    },
                  })
                },
                struct_name => {
                  let struct_ident = Ident::new(&format!("{}", struct_name), Span::def_site());
                  Some(quote!{
                    #label_name => {
                      match #struct_ident::derive_deserialize(read, Some(&attributes)) {
                        Ok(parsed_item) => {
                          prev_level -= 1;
                          #label.push(parsed_item);
                        },
                        Err(msg) => {
                          println!("ERROR {:?}", msg);
                        },
                      }
                    },
                  })
                }
              }
            },
            None => None
          }
        },
        _ => None
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {sum.append_all(val); sum});

  let struct_builder : Tokens = data_struct.fields.iter().map(|ref field|
    {
      let label = field.ident;

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) |
        Some(FieldType::FieldTypeStruct{..}) |
        Some(FieldType::FieldTypeVec) =>
          Some(quote!{
            #label: #label,
          }),
        None => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut tokens, token| {tokens.append_all(token); tokens});

  quote! {
    use xml::reader::XmlEvent;

    impl YaDeserialize for #name {
      #[allow(unused_variables)]
      fn derive_deserialize<R: Read>(read: &mut xml::EventReader<R>, parent_attributes: Option<&Vec<xml::attribute::OwnedAttribute>>) -> Result<Self, String> {
        let mut prev_level = 0;
        let mut current_level = 0;

        #variables
        let current_attributes = parent_attributes;
        #attributes_loading

        loop {
          match read.next() {
            Ok(XmlEvent::StartDocument{..}) => {
            },
            Ok(XmlEvent::EndDocument) => {
              break;
            },
            Ok(XmlEvent::StartElement{name, attributes, namespace: _namespace}) => {
              // println!("{} | {} - {}: {}", #root, prev_level, current_level, name.local_name.as_str());
              if prev_level == current_level {
                match name.local_name.as_str() {
                  #root => {
                    let root_attributes = attributes.clone();
                    let current_attributes = Some(&root_attributes);
                    #attributes_loading

                    current_level += 1;
                  },
                  #fields
                  _ => {}
                };
              }
              
              prev_level += 1;
            },
            Ok(XmlEvent::EndElement{name}) => {
              if #root == name.local_name.as_str() {
                // println!("BREAK {}", #root);
                break;
              }
              prev_level -= 1;
            }
            Ok(xml::reader::XmlEvent::Characters(characters_content)) => {
              if prev_level == current_level {
                #assign_text_field
              }
            },
            Ok(_event) => {
            },
            Err(_msg) => {
              break;
            },
          }
        }
        Ok(#name{#struct_builder})
      }
    }
  }
}
