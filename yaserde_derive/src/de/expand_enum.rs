
use attribute::*;
use field_type::*;
use quote::Tokens;
use syn::Fields;
use syn::Ident;
use syn::DataEnum;
use proc_macro2::Span;

pub fn parse(data_enum: &DataEnum, name: &Ident, root: &String) -> Tokens {
  let variables : Tokens = data_enum.variants.iter().map(|ref variant|
    {
      match variant.fields {
        Fields::Unit => None,
        Fields::Named(ref fields) => {
          let enum_fields = fields.named.iter().map(|ref field| {
            let field_label = field.ident;

            match get_field_type(field) {
              Some(FieldType::FieldTypeString) => {
                Some(quote!{
                  let mut #field_label : String = "".to_string();
                })
              },
              Some(FieldType::FieldTypeStruct{struct_name}) => {
                Some(quote!{
                  let mut #field_label : #struct_name = #struct_name::default();
                })
              },
              Some(FieldType::FieldTypeVec{data_type}) => {
                let dt = Box::into_raw(data_type);
                match unsafe{dt.as_ref()} {
                  Some(&FieldType::FieldTypeString) => {
                    Some(quote!{
                      let mut #field_label : Vec<String> = vec![];
                    })
                  },
                  Some(&FieldType::FieldTypeStruct{struct_name}) => {
                    Some(quote!{
                      let mut #field_label : Vec<#struct_name> = vec![];
                    })
                  },
                  Some(&FieldType::FieldTypeVec{..}) => {unimplemented!();},
                  None => {unimplemented!();},
                }
              },
              None => None
            }

          })
          .filter(|x| x.is_some())
          .map(|x| x.unwrap())
          .fold(Tokens::new(), |mut sum, val| {sum.append_all(val); sum});

          Some(enum_fields)
        }
        Fields::Unnamed(ref _fields) => {
          unimplemented!();
        }
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {sum.append_all(val); sum});

  let fields : Tokens = data_enum.variants.iter().map(|ref variant|
    {
      let variant_attrs = YaSerdeAttribute::parse(&variant.attrs);
      let renamed_variant_label =
        match variant_attrs.rename {
          Some(value) => Ident::new(&format!("{}", value), Span::call_site()),
          None => variant.ident
        };
      let variant_label_name = renamed_variant_label.to_string();

      match variant.fields {
        Fields::Unit => None,
        Fields::Named(ref fields) => {
          let enum_fields = fields.named.iter().map(|ref field| {
            let field_attrs = YaSerdeAttribute::parse(&field.attrs);
            let field_label = field.ident;
            let renamed_field_label =
              match field_attrs.rename {
                Some(value) => Some(Ident::new(&format!("{}", value), Span::call_site())),
                None => field.ident
              };

            let field_label_name = renamed_field_label.unwrap().to_string();
            match get_field_type(field) {
              Some(FieldType::FieldTypeString) => {
                Some(quote!{
                  #variant_label_name => {
                    #[warn(unused_assignments)]
                    let mut local_level = 0;
                    let mut search_local_level = 0;
                    loop{
                      match read.next() {
                        Ok(XmlEvent::StartElement{name, attributes, namespace: _namespace}) => {
                          if name.local_name == #field_label_name {
                            search_local_level += 1
                          }
                          local_level += 1;
                        },
                        Ok(XmlEvent::EndElement{name}) => {
                          local_level -= 1;
                          if name.local_name == #field_label_name {
                            break;
                          }
                        },
                        Ok(xml::reader::XmlEvent::Characters(characters_content)) => {
                          if local_level == 1 && search_local_level == 1 {
                            #field_label = characters_content.trim().to_string();
                          }
                        },
                        _ => {},
                      }
                    }
                  },
                })
              },
              Some(FieldType::FieldTypeStruct{struct_name: _struct_name}) => {
                println!("{:?}", field);
                Some(quote!{
                  #field_label_name => {
                    println!("Start to parse {:?}", #field_label_name);
                    #[warn(unused_assignments)]
                    let mut local_level = 0;
                    let mut search_local_level = 0;

                    loop{
                      match read.next() {
                        Ok(XmlEvent::StartElement{name, attributes, namespace: _namespace}) => {
                          println!("Enum: start element = {:?}", name.local_name.as_str());
                          if name.local_name == #field_label_name {
                            search_local_level += 1
                          }
                          local_level += 1;
                          prev_level += 1;
                        },
                        Ok(XmlEvent::EndElement{name}) => {
                          println!("Enum: end element = {:?}", name.local_name.as_str());
                          local_level -= 1;
                          if name.local_name == #field_label_name {
                            break;
                          }
                        },
                        Ok(xml::reader::XmlEvent::Characters(characters_content)) => {
                          println!("Enum: found value = {:?}", characters_content);
                          if local_level == 1 && search_local_level == 1 {
                            println!("found value = {:?}", characters_content);
                            #field_label = characters_content.trim().to_string();
                          }
                        },
                        _ => {},
                      }
                    }
                  },
                })
              },
              data => {
                println!("{:?}", data);
                None}
            }

          })
          .filter(|x| x.is_some())
          .map(|x| x.unwrap())
          .fold(Tokens::new(), |mut sum, val| {sum.append_all(val); sum});

          Some(enum_fields)
        },
        Fields::Unnamed(ref _fields) => {unimplemented!();}
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {sum.append_all(val); sum});

  let match_to_enum : Tokens = data_enum.variants.iter().map(|ref variant|
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
            #label_name => {
              simple_enum_value = Some(#name::#label);
            }
          })
        },
        Fields::Named(ref _fields) => {
          None
        }
        Fields::Unnamed(ref _fields) => {
          None
        }
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut tokens, token| {tokens.append_all(token); tokens});

  quote!{
    use xml::reader::XmlEvent;

    impl YaDeserialize for #name {
      #[allow(unused_variables)]
      fn derive_deserialize<R: Read>(read: &mut xml::EventReader<R>, parent_attributes: Option<&Vec<xml::attribute::OwnedAttribute>>) -> Result<Self, String> {
        let mut prev_level = 0;
        let mut current_level = 0;
        #[warn(unused_assignments, unused_mut)]
        let mut simple_enum_value = None;

        println!("Enum: start to parse {}",  #root);
        #variables

        loop {
          match read.next() {
            Ok(XmlEvent::StartDocument{..}) => {
            },
            Ok(XmlEvent::EndDocument) => {
              break;
            },
            Ok(XmlEvent::StartElement{name, attributes, namespace: _namespace}) => {
              println!("Enum: {} | {} - {}: {}", #root, prev_level, current_level, name.local_name.as_str());
              if prev_level == current_level {
                match name.local_name.as_str() {
                  #root => {
                    let root_attributes = attributes.clone();
                    let current_attributes = Some(&root_attributes);

                    current_level += 1;
                  },
                  #fields
                  _ => {}
                };
              }

              prev_level += 1;
            },
            Ok(XmlEvent::EndElement{name}) => {
              println!("CLOSE {} | {} - {}: {}", #root, prev_level, current_level, name.local_name.as_str());
              
              if prev_level == current_level {
                println!("LEVEL BREAK {}", #root);
                match simple_enum_value {
                  Some(value) => return Ok(value),
                  None => {
                    return Ok(#name::default());
                  },
                }
              }
              prev_level -= 1;
            },
            Ok(xml::reader::XmlEvent::Characters(characters_content)) => {
              println!("{:?} - {:?} -- {:?}", prev_level, current_level, characters_content.as_str());
              if prev_level == current_level {
                match characters_content.as_str() {
                  #match_to_enum
                  _ => {}
                }
              }
            },
            Ok(event) => {
              println!("{:?}", event);
            },
            Err(_msg) => {
              break;
            },
          }
        }

        match simple_enum_value {
          Some(value) => Ok(value),
          None => {
            Ok(#name::default())
            // Err("unable to load Enum value".to_string())
          },
        }
      }
    }
  }
}
