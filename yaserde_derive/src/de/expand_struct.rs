use attribute::*;
use field_type::*;
use quote::Tokens;
use std::collections::BTreeMap;
use syn::Ident;
use syn::DataStruct;
use proc_macro2::Span;

pub fn parse(
  data_struct: &DataStruct,
  name: &Ident,
  root: &str,
  namespaces: &BTreeMap<String, String>,
) -> Tokens {
  let validate_namespace: Tokens = namespaces
    .iter()
    .map(|(_prefix, namespace)| {
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
    .fold(Tokens::new(), |mut tokens, token| {
      tokens.append_all(token);
      tokens
    });

  let variables: Tokens = data_struct
    .fields
    .iter()
    .map(|field| {
      let label = field.ident;
      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => {
          build_default_value(&label, &quote!{String}, &quote!{"".to_string()})
        }
        Some(FieldType::FieldTypeBool) => {
          build_default_value(&label, &quote!{bool}, &quote!{false})
        }
        Some(FieldType::FieldTypeI8) => build_default_value(&label, &quote!{i8}, &quote!{0}),
        Some(FieldType::FieldTypeU8) => build_default_value(&label, &quote!{u8}, &quote!{0}),
        Some(FieldType::FieldTypeI16) => build_default_value(&label, &quote!{i16}, &quote!{0}),
        Some(FieldType::FieldTypeU16) => build_default_value(&label, &quote!{u16}, &quote!{0}),
        Some(FieldType::FieldTypeI32) => build_default_value(&label, &quote!{i32}, &quote!{0}),
        Some(FieldType::FieldTypeU32) => build_default_value(&label, &quote!{u32}, &quote!{0}),
        Some(FieldType::FieldTypeI64) => build_default_value(&label, &quote!{i64}, &quote!{0}),
        Some(FieldType::FieldTypeU64) => build_default_value(&label, &quote!{u64}, &quote!{0}),
        Some(FieldType::FieldTypeStruct { struct_name }) => Some(quote!{
          #[allow(unused_mut, non_snake_case, non_camel_case_types)]
          let mut #label : #struct_name = #struct_name::default();
        }),
        Some(FieldType::FieldTypeVec { data_type }) => {
          let dt = Box::into_raw(data_type);
          match unsafe { dt.as_ref() } {
            Some(&FieldType::FieldTypeString) => {
              build_default_value(&label, &quote!{Vec<String>}, &quote!{vec![]})
            }
            Some(&FieldType::FieldTypeBool) => {
              build_default_value(&label, &quote!{Vec<bool>}, &quote!{vec![]})
            }
            Some(&FieldType::FieldTypeI8) => {
              build_default_value(&label, &quote!{Vec<i8>}, &quote!{vec![]})
            }
            Some(&FieldType::FieldTypeU8) => {
              build_default_value(&label, &quote!{Vec<u8>}, &quote!{vec![]})
            }
            Some(&FieldType::FieldTypeI16) => {
              build_default_value(&label, &quote!{Vec<i16>}, &quote!{vec![]})
            }
            Some(&FieldType::FieldTypeU16) => {
              build_default_value(&label, &quote!{Vec<u16>}, &quote!{vec![]})
            }
            Some(&FieldType::FieldTypeI32) => {
              build_default_value(&label, &quote!{Vec<i32>}, &quote!{vec![]})
            }
            Some(&FieldType::FieldTypeU32) => {
              build_default_value(&label, &quote!{Vec<u32>}, &quote!{vec![]})
            }
            Some(&FieldType::FieldTypeI64) => {
              build_default_value(&label, &quote!{Vec<i64>}, &quote!{vec![]})
            }
            Some(&FieldType::FieldTypeU64) => {
              build_default_value(&label, &quote!{Vec<u64>}, &quote!{vec![]})
            }
            Some(&FieldType::FieldTypeStruct { struct_name }) => Some(quote!{
              #[allow(unused_mut)]
              let mut #label : Vec<#struct_name> = vec![];
            }),
            Some(&FieldType::FieldTypeVec { .. }) => {
              unimplemented!();
            }
            None => {
              unimplemented!();
            }
          }
        }
        _ => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {
      sum.append_all(val);
      sum
    });

  let field_visitors: Tokens = data_struct
    .fields
    .iter()
    .map(|field| {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      let label_name = if let Some(value) = field_attrs.rename {
        Ident::new(&format!("{}", value), Span::call_site()).to_string()
      } else {
        field.ident.unwrap().to_string()
      };

      let visitor_label = Ident::new(&format!("__Visitor{}", label_name), Span::call_site());

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => {
          let visitor = Ident::new("visit_str", Span::call_site());
          build_declare_visitor(&quote!{String}, &visitor, &visitor_label)
        }
        Some(FieldType::FieldTypeBool) => {
          let visitor = Ident::new("visit_bool", Span::call_site());
          build_declare_visitor(&quote!{bool}, &visitor, &visitor_label)
        }
        Some(FieldType::FieldTypeI8) => {
          let visitor = Ident::new("visit_i8", Span::call_site());
          build_declare_visitor(&quote!{i8}, &visitor, &visitor_label)
        }
        Some(FieldType::FieldTypeU8) => {
          let visitor = Ident::new("visit_u8", Span::call_site());
          build_declare_visitor(&quote!{u8}, &visitor, &visitor_label)
        }
        Some(FieldType::FieldTypeI16) => {
          let visitor = Ident::new("visit_i16", Span::call_site());
          build_declare_visitor(&quote!{i16}, &visitor, &visitor_label)
        }
        Some(FieldType::FieldTypeU16) => {
          let visitor = Ident::new("visit_u16", Span::call_site());
          build_declare_visitor(&quote!{u16}, &visitor, &visitor_label)
        }
        Some(FieldType::FieldTypeI32) => {
          let visitor = Ident::new("visit_i32", Span::call_site());
          build_declare_visitor(&quote!{i32}, &visitor, &visitor_label)
        }
        Some(FieldType::FieldTypeU32) => {
          let visitor = Ident::new("visit_u32", Span::call_site());
          build_declare_visitor(&quote!{u32}, &visitor, &visitor_label)
        }
        Some(FieldType::FieldTypeI64) => {
          let visitor = Ident::new("visit_i64", Span::call_site());
          build_declare_visitor(&quote!{i64}, &visitor, &visitor_label)
        }
        Some(FieldType::FieldTypeU64) => {
          let visitor = Ident::new("visit_u64", Span::call_site());
          build_declare_visitor(&quote!{u64}, &visitor, &visitor_label)
        }
        Some(FieldType::FieldTypeStruct { struct_name }) => {
          let struct_id = struct_name.to_string();
          let struct_ident = Ident::new(
            &format!("__Visitor_{}_{}", label_name, struct_name),
            Span::call_site(),
          );

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
        }
        Some(FieldType::FieldTypeVec { data_type }) => {
          let dt = Box::into_raw(data_type);
          match unsafe { dt.as_ref() } {
            Some(&FieldType::FieldTypeString) => {
              let visitor = Ident::new("visit_str", Span::call_site());
              build_declare_visitor(&quote!{String}, &visitor, &visitor_label)
            }
            Some(&FieldType::FieldTypeBool) => {
              let visitor = Ident::new("visit_bool", Span::call_site());
              build_declare_visitor(&quote!{bool}, &visitor, &visitor_label)
            }
            Some(&FieldType::FieldTypeI8) => {
              let visitor = Ident::new("visit_i8", Span::call_site());
              build_declare_visitor(&quote!{i8}, &visitor, &visitor_label)
            }
            Some(&FieldType::FieldTypeU8) => {
              let visitor = Ident::new("visit_u8", Span::call_site());
              build_declare_visitor(&quote!{u8}, &visitor, &visitor_label)
            }
            Some(&FieldType::FieldTypeI16) => {
              let visitor = Ident::new("visit_i16", Span::call_site());
              build_declare_visitor(&quote!{i16}, &visitor, &visitor_label)
            }
            Some(&FieldType::FieldTypeU16) => {
              let visitor = Ident::new("visit_u16", Span::call_site());
              build_declare_visitor(&quote!{u16}, &visitor, &visitor_label)
            }
            Some(&FieldType::FieldTypeI32) => {
              let visitor = Ident::new("visit_i32", Span::call_site());
              build_declare_visitor(&quote!{i32}, &visitor, &visitor_label)
            }
            Some(&FieldType::FieldTypeU32) => {
              let visitor = Ident::new("visit_u32", Span::call_site());
              build_declare_visitor(&quote!{u32}, &visitor, &visitor_label)
            }
            Some(&FieldType::FieldTypeI64) => {
              let visitor = Ident::new("visit_i64", Span::call_site());
              build_declare_visitor(&quote!{i64}, &visitor, &visitor_label)
            }
            Some(&FieldType::FieldTypeU64) => {
              let visitor = Ident::new("visit_u64", Span::call_site());
              build_declare_visitor(&quote!{u64}, &visitor, &visitor_label)
            }
            Some(&FieldType::FieldTypeStruct { struct_name }) => {
              let struct_ident = Ident::new(&format!("{}", struct_name), Span::def_site());
              Some(quote!{
                #[allow(non_snake_case, non_camel_case_types)]
                struct #visitor_label;
                impl<'de> Visitor<'de> for #visitor_label {
                  type Value = #struct_ident;
                }
              })
            }
            _ => None,
          }
        }
        _ => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {
      sum.append_all(val);
      sum
    });

  let call_visitors: Tokens = data_struct
    .fields
    .iter()
    .map(|field| {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      let label = field.ident;

      if field_attrs.attribute {
        return None;
      }

      let label_name = if let Some(value) = field_attrs.rename {
        Ident::new(&format!("{}", value), Span::call_site()).to_string()
      } else {
        field.ident.unwrap().to_string()
      };

      let visitor_label = Ident::new(&format!("__Visitor{}", label_name), Span::call_site());

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => {
          let visitor = Ident::new("visit_str", Span::call_site());
          build_call_visitor(
            &quote!{String},
            &visitor,
            &quote!{= value},
            &visitor_label,
            &label,
            &label_name,
          )
        }
        Some(FieldType::FieldTypeBool) => {
          let visitor = Ident::new("visit_bool", Span::call_site());
          build_call_visitor(
            &quote!{bool},
            &visitor,
            &quote!{= value},
            &visitor_label,
            &label,
            &label_name,
          )
        }
        Some(FieldType::FieldTypeI8) => {
          let visitor = Ident::new("visit_i8", Span::call_site());
          build_call_visitor(
            &quote!{i8},
            &visitor,
            &quote!{= value},
            &visitor_label,
            &label,
            &label_name,
          )
        }
        Some(FieldType::FieldTypeU8) => {
          let visitor = Ident::new("visit_u8", Span::call_site());
          build_call_visitor(
            &quote!{u8},
            &visitor,
            &quote!{= value},
            &visitor_label,
            &label,
            &label_name,
          )
        }
        Some(FieldType::FieldTypeU16) => {
          let visitor = Ident::new("visit_u16", Span::call_site());
          build_call_visitor(
            &quote!{u16},
            &visitor,
            &quote!{= value},
            &visitor_label,
            &label,
            &label_name,
          )
        }
        Some(FieldType::FieldTypeI16) => {
          let visitor = Ident::new("visit_i16", Span::call_site());
          build_call_visitor(
            &quote!{i16},
            &visitor,
            &quote!{= value},
            &visitor_label,
            &label,
            &label_name,
          )
        }
        Some(FieldType::FieldTypeU32) => {
          let visitor = Ident::new("visit_u32", Span::call_site());
          build_call_visitor(
            &quote!{u32},
            &visitor,
            &quote!{= value},
            &visitor_label,
            &label,
            &label_name,
          )
        }
        Some(FieldType::FieldTypeI32) => {
          let visitor = Ident::new("visit_i32", Span::call_site());
          build_call_visitor(
            &quote!{i32},
            &visitor,
            &quote!{= value},
            &visitor_label,
            &label,
            &label_name,
          )
        }
        Some(FieldType::FieldTypeU64) => {
          let visitor = Ident::new("visit_u64", Span::call_site());
          build_call_visitor(
            &quote!{u64},
            &visitor,
            &quote!{= value},
            &visitor_label,
            &label,
            &label_name,
          )
        }
        Some(FieldType::FieldTypeI64) => {
          let visitor = Ident::new("visit_i64", Span::call_site());
          build_call_visitor(
            &quote!{i64},
            &visitor,
            &quote!{= value},
            &visitor_label,
            &label,
            &label_name,
          )
        }
        Some(FieldType::FieldTypeStruct { struct_name }) => Some(quote!{
          #label_name => {
            reader.set_map_value();
            match #struct_name::deserialize(reader) {
              Ok(parsed_item) => {
                #label = parsed_item;
                let _root = reader.next_event();
              },
              Err(msg) => {
                return Err(msg);
              },
            }
          }
        }),
        Some(FieldType::FieldTypeVec { data_type }) => {
          let dt = Box::into_raw(data_type);
          match unsafe { dt.as_ref() } {
            Some(&FieldType::FieldTypeString) => {
              let visitor = Ident::new("visit_str", Span::call_site());
              build_call_visitor(
                &quote!{String},
                &visitor,
                &quote!{.push(value)},
                &visitor_label,
                &label,
                &label_name,
              )
            }
            Some(&FieldType::FieldTypeBool) => {
              let visitor = Ident::new("visit_bool", Span::call_site());
              build_call_visitor(
                &quote!{bool},
                &visitor,
                &quote!{.push(value)},
                &visitor_label,
                &label,
                &label_name,
              )
            }
            Some(&FieldType::FieldTypeI8) => {
              let visitor = Ident::new("visit_i8", Span::call_site());
              build_call_visitor(
                &quote!{i8},
                &visitor,
                &quote!{.push(value)},
                &visitor_label,
                &label,
                &label_name,
              )
            }
            Some(&FieldType::FieldTypeU8) => {
              let visitor = Ident::new("visit_u8", Span::call_site());
              build_call_visitor(
                &quote!{u8},
                &visitor,
                &quote!{.push(value)},
                &visitor_label,
                &label,
                &label_name,
              )
            }
            Some(&FieldType::FieldTypeI16) => {
              let visitor = Ident::new("visit_i16", Span::call_site());
              build_call_visitor(
                &quote!{i16},
                &visitor,
                &quote!{.push(value)},
                &visitor_label,
                &label,
                &label_name,
              )
            }
            Some(&FieldType::FieldTypeU16) => {
              let visitor = Ident::new("visit_u16", Span::call_site());
              build_call_visitor(
                &quote!{u16},
                &visitor,
                &quote!{.push(value)},
                &visitor_label,
                &label,
                &label_name,
              )
            }
            Some(&FieldType::FieldTypeI32) => {
              let visitor = Ident::new("visit_i32", Span::call_site());
              build_call_visitor(
                &quote!{i32},
                &visitor,
                &quote!{.push(value)},
                &visitor_label,
                &label,
                &label_name,
              )
            }
            Some(&FieldType::FieldTypeU32) => {
              let visitor = Ident::new("visit_u32", Span::call_site());
              build_call_visitor(
                &quote!{u32},
                &visitor,
                &quote!{.push(value)},
                &visitor_label,
                &label,
                &label_name,
              )
            }
            Some(&FieldType::FieldTypeI64) => {
              let visitor = Ident::new("visit_i64", Span::call_site());
              build_call_visitor(
                &quote!{i64},
                &visitor,
                &quote!{.push(value)},
                &visitor_label,
                &label,
                &label_name,
              )
            }
            Some(&FieldType::FieldTypeU64) => {
              let visitor = Ident::new("visit_u64", Span::call_site());
              build_call_visitor(
                &quote!{u64},
                &visitor,
                &quote!{.push(value)},
                &visitor_label,
                &label,
                &label_name,
              )
            }
            Some(&FieldType::FieldTypeStruct { struct_name }) => {
              let struct_ident = Ident::new(&format!("{}", struct_name), Span::def_site());
              Some(quote!{
                #label_name => {
                  reader.set_map_value();
                  match #struct_ident::deserialize(reader) {
                    Ok(parsed_item) => {
                      #label.push(parsed_item);
                      let _root = reader.next_event();
                    },
                    Err(msg) => {
                      return Err(msg);
                    },
                  }
                }
              })
            }
            _ => unimplemented!(),
          }
        }
        _ => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {
      sum.append_all(val);
      sum
    });

  let attributes_loading: Tokens = data_struct
    .fields
    .iter()
    .map(|field| {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      if !field_attrs.attribute {
        return None;
      }

      let label = field.ident;
      let label_name = if let Some(value) = field_attrs.rename {
        Ident::new(&format!("{}", value), Span::call_site()).to_string()
      } else {
        field.ident.unwrap().to_string()
      };

      let visitor_label = Ident::new(&format!("__Visitor{}", label_name), Span::call_site());

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => Some(quote!{
          for attr in attributes {
            if attr.name.local_name == #label_name {
              #label = attr.value.to_owned();
            }
          }
        }),
        Some(FieldType::FieldTypeBool) => build_call_visitor_for_attribute(&label, &label_name, &quote!{visit_bool}, &visitor_label),
        Some(FieldType::FieldTypeI8) => build_call_visitor_for_attribute(&label, &label_name, &quote!{visit_i8}, &visitor_label),
        Some(FieldType::FieldTypeU8) => build_call_visitor_for_attribute(&label, &label_name, &quote!{visit_u8}, &visitor_label),
        Some(FieldType::FieldTypeI16) => build_call_visitor_for_attribute(&label, &label_name, &quote!{visit_i16}, &visitor_label),
        Some(FieldType::FieldTypeU16) => build_call_visitor_for_attribute(&label, &label_name, &quote!{visit_u16}, &visitor_label),
        Some(FieldType::FieldTypeI32) => build_call_visitor_for_attribute(&label, &label_name, &quote!{visit_i32}, &visitor_label),
        Some(FieldType::FieldTypeU32) => build_call_visitor_for_attribute(&label, &label_name, &quote!{visit_u32}, &visitor_label),
        Some(FieldType::FieldTypeI64) => build_call_visitor_for_attribute(&label, &label_name, &quote!{visit_i64}, &visitor_label),
        Some(FieldType::FieldTypeU64) => build_call_visitor_for_attribute(&label, &label_name, &quote!{visit_u64}, &visitor_label),

        Some(FieldType::FieldTypeStruct { struct_name }) => {
          let struct_ident = Ident::new(
            &format!("__Visitor_{}_{}", label_name, struct_name),
            Span::call_site(),
          );

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
        _ => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut sum, val| {
      sum.append_all(val);
      sum
    });

  let set_text: Tokens = data_struct
    .fields
    .iter()
    .map(|field| {
      let label = field.ident;
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => {
          build_set_text_to_value(&field_attrs, &label, &quote!{text_content.to_owned()})
        }
        Some(FieldType::FieldTypeBool) => build_set_text_to_value(
          &field_attrs,
          &label,
          &quote!{bool::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeI8) => build_set_text_to_value(
          &field_attrs,
          &label,
          &quote!{i8::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeU8) => build_set_text_to_value(
          &field_attrs,
          &label,
          &quote!{u8::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeI16) => build_set_text_to_value(
          &field_attrs,
          &label,
          &quote!{i16::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeU16) => build_set_text_to_value(
          &field_attrs,
          &label,
          &quote!{u16::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeI32) => build_set_text_to_value(
          &field_attrs,
          &label,
          &quote!{i32::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeU32) => build_set_text_to_value(
          &field_attrs,
          &label,
          &quote!{u32::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeI64) => build_set_text_to_value(
          &field_attrs,
          &label,
          &quote!{i64::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeU64) => build_set_text_to_value(
          &field_attrs,
          &label,
          &quote!{u64::from_str(text_content).unwrap()},
        ),

        Some(FieldType::FieldTypeStruct { .. }) | Some(FieldType::FieldTypeVec { .. }) | None => {
          None
        }
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut tokens, token| {
      tokens.append_all(token);
      tokens
    });

  let struct_builder: Tokens = data_struct
    .fields
    .iter()
    .map(|field| {
      let label = field.ident;

      if get_field_type(field).is_some() {
        Some(quote!{
          #label: #label,
        })
      } else {
        None
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(Tokens::new(), |mut tokens, token| {
      tokens.append_all(token);
      tokens
    });

  quote! {
    use xml::reader::XmlEvent;
    use yaserde::Visitor;
    #[allow(unknown_lints, unused_imports)]
    use std::str::FromStr;

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
                  let _root = reader.next_event();
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
              let _root = reader.next_event();
            }
            XmlEvent::Characters(ref text_content) => {
              #set_text
              let _root = reader.next_event();
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

fn build_default_value(
  label: &Option<Ident>,
  field_type: &Tokens,
  default: &Tokens,
) -> Option<Tokens> {
  Some(quote!{
    #[allow(unused_mut)]
    let mut #label : #field_type = #default;
  })
}

fn build_declare_visitor(
  field_type: &Tokens,
  visitor: &Ident,
  visitor_label: &Ident,
) -> Option<Tokens> {
  Some(quote!{
    #[allow(non_snake_case, non_camel_case_types)]
    struct #visitor_label;
    impl<'de> Visitor<'de> for #visitor_label {
      type Value = #field_type;

      fn #visitor(self, v: &str) -> Result<Self::Value, String> {
        Ok(#field_type::from_str(v).unwrap())
      }
    }
  })
}

fn build_call_visitor(
  field_type: &Tokens,
  visitor: &Ident,
  action: &Tokens,
  visitor_label: &Ident,
  label: &Option<Ident>,
  label_name: &str,
) -> Option<Tokens> {
  Some(quote!{
    #label_name => {
      let visitor = #visitor_label{};

      if let XmlEvent::StartElement { .. } = *reader.peek()? {
        reader.set_map_value()
      }

      let result = reader.read_inner_value::<#field_type, _>(|reader| {
        if let XmlEvent::EndElement { .. } = *reader.peek()? {
          return visitor.#visitor("");
        }

        if let Ok(XmlEvent::Characters(s)) = reader.next_event() {
          visitor.#visitor(&s)
        } else {
          Err(format!("unable to parse content for {}", #label_name))
        }
      });

      if let Ok(value) = result {
        #label#action
      }
    }
  })
}

fn build_call_visitor_for_attribute(
  label: &Option<Ident>,
  label_name: &str,
  visitor: &Tokens,
  visitor_label: &Ident,
) -> Option<Tokens> {
  Some(quote!{
    for attr in attributes {
      if attr.name.local_name == #label_name {
        let visitor = #visitor_label{};
        match visitor.#visitor(&attr.value) {
          Ok(value) => {#label = value;}
          Err(msg) => {return Err(msg);}
        }
      }
    }
  })
}

fn build_set_text_to_value(
  field_attrs: &YaSerdeAttribute,
  label: &Option<Ident>,
  action: &Tokens,
) -> Option<Tokens> {
  if field_attrs.text {
    Some(quote!{
      #label = #action;
    })
  } else {
    None
  }
}
