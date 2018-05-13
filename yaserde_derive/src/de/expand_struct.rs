
use attribute::*;
use field_type::*;
use quote::Tokens;
use std::collections::BTreeMap;
use syn::Ident;
use syn::DataStruct;
use proc_macro2::Span;

pub fn parse(data_struct: &DataStruct, name: &Ident, root: &String, namespaces: &BTreeMap<String, String>) -> Tokens {

  let validate_namespace : Tokens = namespaces.iter().map(|(ref prefix, ref namespace)| {
      Some(quote!(

        let mut found = false;
        for (key, value) in namespace {
          if #namespace == value {
            found = true;
          }
        }
        if !found {
          return Err("bad namespace".to_string());
        }
        // println!("{}: {}", #prefix, #namespace);
      ))
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut tokens, token| {tokens.append_all(token); tokens});


  let variables: Tokens = data_struct.fields.iter().map(|ref field|
    {
      let label = field.ident;
      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => {
          Some(quote!{
            #[allow(unused_mut)]
            let mut #label : String = "".to_string();
          })
        },
        Some(FieldType::FieldTypeStruct{struct_name}) => {
          Some(quote!{
            #[allow(unused_mut, non_snake_case, non_camel_case_types)]
            let mut #label : #struct_name = #struct_name::default();
          })
        },
        Some(FieldType::FieldTypeVec{data_type}) => {
          let dt = Box::into_raw(data_type);
          match unsafe{dt.as_ref()} {
            Some(&FieldType::FieldTypeString) => {
              Some(quote!{
                #[allow(unused_mut)]
                let mut #label : Vec<String> = vec![];
              })
            },
            Some(&FieldType::FieldTypeStruct{struct_name}) => {
              Some(quote!{
                #[allow(unused_mut)]
                let mut #label : Vec<#struct_name> = vec![];
              })
            },
            Some(&FieldType::FieldTypeVec{..}) => {unimplemented!();},
            None => {unimplemented!();},
          }
        },
        _ => None
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {sum.append_all(val); sum});

  let field_visitors: Tokens = data_struct.fields.iter().map(|ref field|
    {
      let label = field.ident;
      let label_name = label.unwrap().to_string();
      let visitor_label = Ident::new(&format!("__Visitor{}", label_name), Span::call_site());

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => {
          Some(quote!{
            #[allow(non_snake_case, non_camel_case_types)]
            struct #visitor_label;
            impl<'de> Visitor<'de> for #visitor_label {
              type Value = String;

              fn visit_str(self, v: &str) -> Result<Self::Value, String> {
                Ok(String::from(v))
              }
            }
          })
        },
        Some(FieldType::FieldTypeStruct{struct_name}) => {
          let struct_id = struct_name.to_string();
          let struct_ident = Ident::new(&format!("__Visitor_{}_{}", label_name, struct_name), Span::call_site());

          Some(quote!{
            #[allow(non_snake_case, non_camel_case_types)]
            struct #struct_ident;
            impl<'de> Visitor<'de> for #struct_ident {
              type Value = #struct_name;

              fn visit_str(self, v: &str) -> Result<Self::Value, String> {
                let content = "<".to_string() + #struct_id + ">" + v + "</" + #struct_id + ">";
                let value : Result<#struct_name, String> = yaserde::de::from_str(&content);
                value
              }
            }
          })
        },
        Some(FieldType::FieldTypeVec{data_type}) => {
          let dt = Box::into_raw(data_type);
          match unsafe{dt.as_ref()} {
            Some(&FieldType::FieldTypeString) => {
              Some(quote!{
                #[allow(non_snake_case, non_camel_case_types)]
                struct #visitor_label;
                impl<'de> Visitor<'de> for #visitor_label {
                  type Value = String;

                  fn visit_str(self, v: &str) -> Result<Self::Value, String> {
                    Ok(String::from(v))
                  }
                }
              })
            }
            Some(&FieldType::FieldTypeStruct{struct_name}) => {
              let struct_ident = Ident::new(&format!("{}", struct_name), Span::def_site());
              Some(quote!{
                #[allow(non_snake_case, non_camel_case_types)]
                struct #visitor_label;
                impl<'de> Visitor<'de> for #visitor_label {
                  type Value = #struct_ident;
                }
              })
            }
            _ => {
              None
            }
          }
        },
        _ => None
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {sum.append_all(val); sum});

  let call_visitors: Tokens = data_struct.fields.iter().map(|ref field|
    {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      let label = field.ident;

      if field_attrs.attribute {
        return None;
      }

      let label_name =
        if let Some(value) = field_attrs.rename {
          Ident::new(&format!("{}", value), Span::call_site()).to_string()
        } else {
          field.ident.unwrap().to_string()
        };

      let visitor_label = Ident::new(&format!("__Visitor{}", label_name), Span::call_site());

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => {
          Some(quote!{
            #label_name => {
              let visitor = #visitor_label{};

              if let XmlEvent::StartElement { .. } = *reader.peek()? {
                reader.set_map_value()
              }

              let result = reader.read_inner_value::<String, _>(|reader| {
                if let XmlEvent::EndElement { .. } = *reader.peek()? {
                  return visitor.visit_str("");
                }

                if let Ok(XmlEvent::Characters(s)) = reader.next() {
                  visitor.visit_str(&s)
                } else {
                  Err(format!("unable to parse content for {}", #label_name))
                }
              });

              if let Ok(value) = result {
                #label = value
              }
            }
          })
        },
        Some(FieldType::FieldTypeStruct{struct_name}) => {
          Some(quote!{
            #label_name => {
              reader.set_map_value();
              match #struct_name::deserialize(reader) {
                Ok(parsed_item) => {
                  #label = parsed_item;
                  let _root = reader.next();
                },
                Err(msg) => {
                  return Err(msg);
                },
              }
            }
          })
        },
        Some(FieldType::FieldTypeVec{data_type}) => {
          let dt = Box::into_raw(data_type);
          match unsafe{dt.as_ref()} {
            Some(&FieldType::FieldTypeString) => {
              Some(quote!{
                #label_name => {
                  let visitor = #visitor_label{};
                  if let XmlEvent::StartElement { .. } = *reader.peek()? {
                    reader.set_map_value()
                  }

                  let result = reader.read_inner_value::<String, _>(|reader| {
                    if let XmlEvent::EndElement { .. } = *reader.peek()? {
                      return visitor.visit_str("");
                    }

                    if let Ok(XmlEvent::Characters(s)) = reader.next() {
                      visitor.visit_str(&s)
                    } else {
                      Err(format!("unable to parse content for {}", #label_name))
                    }
                  });

                  if let Ok(value) = result {
                    #label.push(value)
                  }
                }
              })
            }
            Some(&FieldType::FieldTypeStruct{struct_name}) => {
              let struct_ident = Ident::new(&format!("{}", struct_name), Span::def_site());
              Some(quote!{
                #label_name => {
                  reader.set_map_value();
                  match #struct_ident::deserialize(reader) {
                    Ok(parsed_item) => {
                      #label.push(parsed_item);
                      let _root = reader.next();
                    },
                    Err(msg) => {
                      return Err(msg);
                    },
                  }
                }
              })
            }
            _ => unimplemented!()
          }
        },
        _ => None
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {sum.append_all(val); sum});

  let attributes_loading: Tokens = data_struct.fields.iter().map(|ref field| {
    let field_attrs = YaSerdeAttribute::parse(&field.attrs);
    if !field_attrs.attribute {
      return None;
    }

    let label = field.ident;
    let field_ident = field.ident.unwrap().to_string();
    let label_name =
      if let Some(value) = field_attrs.rename {
        Ident::new(&format!("{}", value), Span::call_site()).to_string()
      } else {
        field.ident.unwrap().to_string()
      };

    match get_field_type(field) {
      Some(FieldType::FieldTypeString) => {
        Some(quote!{
          for attr in attributes {
            if attr.name.local_name == #label_name {
              #label = attr.value.to_owned();
            }
          }
        })
      }
      Some(FieldType::FieldTypeStruct{struct_name}) => {
        let struct_ident = Ident::new(&format!("__Visitor_{}_{}", field_ident, struct_name), Span::call_site());

        Some(quote!{
          for attr in attributes {
            if attr.name.local_name == #label_name {
              let visitor = #struct_ident{};
              match visitor.visit_str(&attr.value) {
                Ok(value) => {#label = value;}
                Err(msg) => {return Err(msg);}
              }
            }
          }
        })
      }
      _ => {
        None
      }
    }})
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {sum.append_all(val); sum});

  let set_text: Tokens = data_struct.fields.iter().map(|ref field|
    {
      let label = field.ident;
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => {
          if field_attrs.text {
            Some(quote!{
              #label = text_content.to_owned();
            })
          } else {
            None
          }
        },
        Some(FieldType::FieldTypeStruct{..}) |
        Some(FieldType::FieldTypeVec{..})|
        None => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut tokens, token| {tokens.append_all(token); tokens});

  let struct_builder: Tokens = data_struct.fields.iter().map(|ref field|
    {
      let label = field.ident;

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) |
        Some(FieldType::FieldTypeStruct{..}) |
        Some(FieldType::FieldTypeVec{..}) =>
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
    use yaserde::Visitor;

    impl YaDeserialize for #name {
      #[allow(unused_variables)]
      fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        let named_element =
          if let XmlEvent::StartElement{name, ..} = reader.peek()?.to_owned() {
            name.local_name.to_owned()
          } else {
            String::from(#root)
          };
        debug!("Struct: start to parse {:?}", named_element);

        #variables
        #field_visitors

        loop {
          match reader.peek()?.to_owned() {
            XmlEvent::StartElement{ref name, ref attributes, ref namespace} => {
              #validate_namespace

              match name.local_name.as_str() {
                #call_visitors
                named_element => {
                  let _root = reader.next();
                }
                // name => {
                //   return Err(format!("unknown key {}", name))
                // }
              }
              #attributes_loading
            }
            XmlEvent::EndElement{ref name} => {
              if name.local_name == named_element {
                break;
              }
              let _root = reader.next();
            }
            XmlEvent::Characters(ref text_content) => {
              #set_text
              let _root = reader.next();
            }
            event => {
              return Err(format!("unknown event {:?}", event))
            }
          }
        }

        Ok(#name{#struct_builder})
      }
    }
  }
}
